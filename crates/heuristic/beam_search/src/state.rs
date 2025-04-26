use crate::evaluation::Evaluation;

pub trait BeamState {
    type Action: Clone + Default;

    fn enumerate_actions(&mut self) -> Vec<Self::Action>;
    fn apply_action(&mut self, action: &Self::Action);
    fn revert_action(&mut self, action: &Self::Action);

    fn action_turns(&self, action: &Self::Action) -> usize;

    fn evaluate_current_state(&mut self) -> Evaluation {
        unimplemented!("evaluate_current_state か evaluate_after_action を実装してください")
    }

    fn evaluate_after_action(&mut self, action: &Self::Action) -> Evaluation {
        self.apply_action(action);
        let evaluation = self.evaluate_current_state();
        self.revert_action(action);
        evaluation
    }
}
