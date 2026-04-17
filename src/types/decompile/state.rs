use crate::types::structs::{DLVectorReference, State};
use crate::utils::binary_reader::{assert_values, BinaryReader};
use crate::utils::important_data::ImportantData;
use crate::utils::text_writer::TextWriter;

impl State {
    pub fn read(br: &mut BinaryReader, important_data: &mut ImportantData) -> State {
        let struct_type = br.read_uint16();
        assert_eq!(important_data.get_type_name(struct_type), "EzStateMapState");

        let version = br.read_uint32();
        assert_values(&version, vec![1, 2]);

        let state_index = br.read_uint32();
        let transitions = DLVectorReference::read(br, important_data);
        let entry_events = DLVectorReference::read(br, important_data);
        let exit_events = DLVectorReference::read(br, important_data);

        State {
            struct_type,
            version,
            state_index,
            transitions,
            entry_events,
            exit_events
        }
    }
    
    pub fn decompile(&self, tw: &mut TextWriter, important_data: &ImportantData) {
        tw.write_line(format!("    def State_{}():", self.state_index));
        
        if self.entry_events.ref_count != 0 {
            tw.write_line("        #EntryEvents:".to_string());
            self.entry_events.decompile(tw, important_data);
        }

        if self.exit_events.ref_count != 0 {
            tw.write_line("        #ExitEvents:".to_string());
            self.exit_events.decompile(tw, important_data);
        }
        
        if self.transitions.ref_count != 0 {
            tw.write_line("        #Transitions:".to_string());
            self.transitions.decompile(tw, important_data);
        }
    }
}