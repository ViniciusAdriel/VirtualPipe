use std::rc::Rc;
use slint::{VecModel, Model};
use crate::Pipe;

pub fn get_suffix(
    pipelist: &Rc<VecModel<Pipe>>,
    default_sink_name: &str,
    default_source_name: &str
)-> i32
{
    let mut suffix = 0;

    loop {
        let s = suffix.to_string();

        let sink_name_candidate   = format!(
            "{default_sink_name}{}", if suffix == 0 {""} else {&s}
        );
        let source_name_candidate = format!(
            "{default_source_name}{}", if suffix == 0 {""} else {&s}
        );

        if !(0..pipelist.row_count())
            .filter_map(|i| pipelist.row_data(i))
            .any(|pipe| pipe.sink == sink_name_candidate)
        || !(0..pipelist.row_count())
            .filter_map(|i| pipelist.row_data(i))
            .any(|pipe| pipe.source == source_name_candidate)
        {
            return suffix;
        } else {
            suffix += 1;
            continue;
        }
    }
}