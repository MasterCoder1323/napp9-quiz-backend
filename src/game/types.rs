use fastrand;
use serde::Serialize;

#[derive(Debug, Clone, Copy)]
pub struct Question {
    pub text: &'static str,
    pub options: [&'static str; 4], // first is correct
}

#[derive(Debug, Clone, Serialize)]
pub struct ShuffledQuestion {
    pub text: &'static str,
    pub options: [String; 4], // shuffled options
}

#[derive(Debug)]
pub struct QuestionList {
    pub questions: &'static [Question],
}

impl QuestionList {
    /// Pick a random question, shuffle options, return new struct + correct index
    pub fn select_random_question(&self) -> (ShuffledQuestion, usize) {
        let question_index = fastrand::usize(..self.questions.len());
        let question = &self.questions[question_index];

        // Shuffle options using fastrand
        let mut shuffled = question.options;
        for i in (1..shuffled.len()).rev() {
            let j = fastrand::usize(..=i);
            shuffled.swap(i, j);
        }

        // Find new index of correct answer
        let correct_index = shuffled
            .iter()
            .position(|&o| o == question.options[0])
            .unwrap();

        let shuffled_array = [
            shuffled[0].to_string(),
            shuffled[1].to_string(),
            shuffled[2].to_string(),
            shuffled[3].to_string(),
        ];

        let new_question = ShuffledQuestion {
            text: question.text,
            options: shuffled_array,
        };

        (new_question, correct_index)
    }
}

/// Check answer by index
pub fn check_answer(selected_index: usize, correct_index: usize) -> bool {
    selected_index == correct_index
}
