use std::rc::Rc;
use slint::{VecModel, Model};
use crate::Pipe;

pub fn get_default_pipe_name(
    pipelist: &Rc<VecModel<Pipe>>,
    default_sink_name: &str,
    default_source_name: &str
)-> (String, String) {
    let sink_name;
    let source_name;

    let mut suffix = 1;

    loop {
        let sink_name_candidate   = format!("{default_sink_name}{suffix}");
        let source_name_candidate = format!("{default_source_name}{suffix}");

        if !(0..pipelist.row_count())
            .filter_map(|i| pipelist.row_data(i))
            .any(|pipe| pipe.sink == sink_name_candidate)
        || !(0..pipelist.row_count())
            .filter_map(|i| pipelist.row_data(i))
            .any(|pipe| pipe.source == source_name_candidate)
        {
            sink_name   = sink_name_candidate;
            source_name = source_name_candidate;
            break;
        } else {
            suffix += 1;
            continue;
        }
    }

    (sink_name, source_name)
}