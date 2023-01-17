pub struct CompletionState {
    input_value: String,
}

impl Default for CompletionState {
    fn default() -> Self {
        CompletionState {
            input_value: Default::default(),
        }
    }
}

impl CompletionState {
    pub fn input_change(&mut self, new_input: &str) {
        self.input_value = new_input.into();
    }
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::wasm_bindgen_test;

    use super::CompletionState;

    #[wasm_bindgen_test]
    fn test_state_changes_its_input_value_on_input_change() {
        let mut state: CompletionState = Default::default();
        let new_input = "input";

        assert_ne!(state.input_value, new_input);

        state.input_change(new_input);
        assert_eq!(state.input_value, new_input)
    }
}
