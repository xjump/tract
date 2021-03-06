use crate::model::{InletId, Model, OutletId, TensorInfo};
use crate::prelude::*;
use std::collections::HashMap;
use std::convert::TryFrom;

pub(crate) fn compact<TI1, TI2, E>(old: &Model<TI1>) -> TractResult<Model<TI2>>
where
    TractError: From<E>,
    TI1: TensorInfo + Clone,
    TI2: TensorInfo + TryFrom<TI1, Error=E>,
{
    let mut model = Model::default();
    let mut map = HashMap::new();
    for old_id in old.eval_order()? {
        let old_node = &old.nodes()[old_id];
        let facts = old_node
            .outputs
            .iter()
            .map(|of| Ok(TI2::try_from(of.fact.clone())?))
            .collect::<TractResult<TVec<_>>>()
            .map_err(|e| format!("While translating {}: {:?}", old_node, e))?;
        let new_id = model.add_node(old_node.name.clone(), old_node.op.clone(), facts)?;
        map.insert(old_id, new_id);
        if old.input_outlets()?.contains(&OutletId::new(old_node.id, 0)) {
            continue;
        }
        for (ix, input) in old_node.inputs.iter().enumerate() {
            model
                .add_edge(OutletId::new(map[&input.node], input.slot), InletId::new(new_id, ix))?;
        }
    }
    // maintaining order of i/o interface
    model.inputs = old
        .input_outlets()?
        .iter()
        .filter_map(|i| map.get(&i.node).map(|&n| OutletId::new(n, i.slot)))
        .collect();
    model.outputs =
        old.output_outlets()?.iter().map(|o| OutletId::new(map[&o.node], o.slot)).collect();
    Ok(model)
}
