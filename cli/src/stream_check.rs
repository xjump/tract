use colored::Colorize;
use simplelog::Level::Info;
use ndarray::Axis;
use tfdeploy::analyser::TensorFact;
use tfdeploy::streaming::StreamingPlan;
use tfdeploy::tensor::Datum;
use tfdeploy::ops::StepValue;
use tfdeploy::{SimplePlan, Tensor};
use {OutputParameters, Parameters};
use errors::*;

pub fn handle(params: Parameters, output_params: OutputParameters) -> Result<()> {
    let model = params.tfd_model;
    let input = params.input.as_ref().unwrap();

    // First generate random values for the inputs.
    let fixed_input = input.to_tensor_with_stream_dim(Some(500))?;
    let wanted_matches = 20;

    let regular_input_fact = TensorFact::default()
        .with_shape(fixed_input.shape())
        .with_datum_type(fixed_input.datum_type());

    // streaming model
    let stream_model = model
        .analyser(&params.output_node)?
        .with_hint(&params.input_nodes[0], &input.to_fact())?
        .to_optimized_model()?;

    // batch model
    let batch_model = model
        .analyser(&params.output_node)?
        .with_hint(&params.input_nodes[0], &regular_input_fact)?
        .to_optimized_model()?;

    let mut analyser = stream_model
        .analyser(&params.output_node)?
        .with_hint(&params.input_nodes[0], &input.to_fact())?;
    analyser.analyse()?;

    let mut display_graph =
        ::display_graph::DisplayGraph::from_nodes(&stream_model.nodes)?
        .with_graph_def(&params.graph)?
        .with_analyser(&analyser)?;

    let eval_order = ::tfdeploy::model::eval_order_for_nodes(&stream_model.nodes(), &[stream_model.node_by_name(&params.output_node)?.id])?;

    for mut dn in &mut display_graph.nodes {
        dn.hidden = true;
    }

    // plan and state for reference batch mode
    let batch_plan = SimplePlan::new(&batch_model, &params.input_nodes, &[&params.output_node])?;
    let mut batch_state = batch_plan.state()?;
    batch_state.set_input(0, fixed_input.clone())?;

    let mut failure = false;

    for node in eval_order.iter() {
        let dn = &mut display_graph.nodes[*node];
        let node = &stream_model.nodes()[*node];
        println!("node: {:?}", node);

        if node.op_name == "Placeholder" || node.op_name == "Const" {
            continue;
        }

        let batch_node = &batch_model.node_by_name(&node.name)?;
        batch_state.compute_recursively(batch_node.id)?;
        let batch_expected = &batch_state.values[batch_node.id].as_ref().unwrap()[0];
        let out_edge_id = analyser.next_edges[node.id][0];
        let out_edge_fact = &analyser.edges[out_edge_id].fact;
        let out_stream_dim = out_edge_fact.streaming_dim()?.unwrap();

        println!("expected: {:?}", batch_expected.shape());
        for line in batch_expected.as_f32s().unwrap().axis_chunks_iter(Axis(out_stream_dim), 1).take(10) {
            println!("  expected: {:?}", line.iter().take(5).cloned().collect::<Vec<f32>>());
        }

        let mut batch_expected = batch_expected.as_f32s().unwrap()
            .axis_chunks_iter(Axis(out_stream_dim), 1);

        // Run streaming node
        let op = &node.op;
        let mut buffers = op.new_buffer();

        let edges:Vec<_> = analyser.prev_edges[node.id].iter().map(|id| &analyser.edges[*id]).collect();

        let mut input_offset = 0;
        let mut lines = vec!();
        let mut matched = 0;

        'stream:
        loop {
            let mut inputs = vec!();
            for edge in &edges {
                if let Some(sdim) = edge.fact.streaming_dim()? {
                    let prec_name = &stream_model.nodes()[edge.from_node.unwrap()].name;
                    let batch_prec_node = batch_state.model().node_by_name(&prec_name)?;
                    let data = &batch_state.values[batch_prec_node.id].as_ref().unwrap()[edge.from_out];
                    let data = data.as_f32s().unwrap();
                    let chunk = data.axis_chunks_iter(Axis(sdim), 1).skip(input_offset).next().unwrap();
                    inputs.push(StepValue::Stream(sdim, Some(Tensor::from(chunk.to_owned()).into())));
                } else {
                    let value = stream_model.nodes[edge.from_node.unwrap()].op().const_value().ok_or("Not a const")?;
                    inputs.push(StepValue::Const(value.into()))
                }
            }
            let output = op.step(inputs, &mut buffers)?;
            input_offset += 1;
            let output = if let Some(output) = output { output } else { continue };
            let found: &Tensor = &output[0];
            let found = found.as_f32s().unwrap();
            for found in found.axis_chunks_iter(Axis(out_stream_dim), 1) {
                let found: Tensor = found.to_owned().into();
                lines.push(format!("found: {:?}", found));
                if let Some(expected) = batch_expected.next() {
                    lines.push(format!("expected: {:?}", Tensor::from(expected.to_owned())));
                    lines.push("".into());
                    let expected = expected.to_owned().into();
                    if found.close_enough(&expected, op.rounding_errors()) {
                        matched += 1;
                        if matched == wanted_matches {
                            break 'stream;
                        }
                    } else {
                        use itertools::Itertools;
                        println!("found: {:?}", found.as_f32s().unwrap().iter().take(5).join(" "));
                    //    break 'stream;
                    }
                } else {
                    break 'stream;
                }
            }
        }
        if matched == wanted_matches {
            dn.label = Some("OK".green().to_string());
        } else {
            dn.label = Some("MISMATCH".red().to_string());
            dn.hidden = false;
            dn.more_lines.push(format!("matched {} records streaming dim {:?}", matched, out_edge_fact.streaming_dim()?));
            dn.more_lines.extend(lines.into_iter());
            failure = true;
            panic!();
        }
    }

    if failure {
        display_graph.render(&output_params)?;
    } else if log_enabled!(Info) {
        display_graph.render(&output_params)?;
    } else {
        println!("{}", "Each node passed the comparison.".bold().green());
    }
    Ok(())
}