use std::sync::Arc;
#[derive(Debug)]
pub struct SharedString {
    pub length: u32,
    pub str: String
}
#[derive(Debug)]
pub struct Evaluator {
    pub struct_type: u16,
    pub version: u32,
    pub buffer: Buffer
}
#[derive(Debug)]
pub struct Buffer {
    pub struct_type: u16,
    pub length: u32,
    pub data: Vec<u8>
}
#[derive(Debug)]
pub struct Header {
    pub magic: String,
    pub unk04: u16,
    pub version: u16,
    pub unk08: Option<u32>,
    pub transition_count: Option<u32>,
    pub highest_ref_id_plus_one: Option<u32>,
    pub string_count: u16,
    pub strings: Vec<SharedString>
}
#[derive(Debug)]
pub struct Project {
    pub struct_type: u16,
    pub version: u32,
    pub maps: DLVectorReference
}
#[derive(Debug)]
pub struct Map {
    pub struct_type: u16,
    pub version: u32,
    pub map_index: u32,
    pub initial_state: Reference,
    pub states: DLVectorReference,
    pub transitions: Option<DLVectorReference>
}
#[derive(Debug)]
pub struct State {
    pub struct_type: u16,
    pub version: u32,
    pub state_index: u32,
    pub transitions: DLVectorReference,
    pub entry_events: DLVectorReference,
    pub exit_events: DLVectorReference
}
#[derive(Debug)]
pub struct Event {
    pub struct_type: u16,
    pub version: u32,
    pub event_id: u32,
    pub arg_amount: u32,
    pub arg_evaluators: Option<Vec<Evaluator>>,
    pub arg_buffers: Option<Vec<Buffer>>
}
#[derive(Debug)]
pub struct Transition {
    pub struct_type: u16,
    pub version: u32,
    pub cond_ref: Option<Reference>,
    pub target_state: Option<Reference>,
    pub cond_buffer: Option<Buffer>,
    pub pass_events: DLVectorReference
}
#[derive(Debug)]
pub struct Condition {
    pub struct_type: u16,
    pub version: u32,
    pub evaluator: Evaluator
}
#[derive(Debug)]
pub struct DLVectorReference {
    pub struct_type: u16,
    pub ref_count: u32,
    pub item_type: Option<u16>,
    pub references: Vec<Reference>
}
#[derive(Debug)]
pub enum RefValue {
    Map(Map),
    State(State),
    Event(Event),
    Transition(Transition),
    Condition(Condition)
}
#[derive(Debug)]
pub struct Reference {
    pub ref_type: u16,
    pub id: u32,
    pub value: Option<Arc<RefValue>>
}