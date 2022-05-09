use crate::model::{SlackBlockActionsState, LunchSpot, SlackElementType};
use std::collections::HashMap;
use std::error::Error;
use chrono::Weekday;
use crate::error::ValidationError;

/// build a new lunch spot form the lunch spot add form
/// ### params
/// action_state - the lunch spot add submit form
pub fn create_lunch_spot_from_form(action_state: &Vec<SlackBlockActionsState>) -> Result<LunchSpot, Box<dyn Error + Send>> {
    let mut lunch_spot_name: Option<String> = None;
    let mut open_days: HashMap<String, bool> = HashMap::new();

    action_state.iter().for_each(|state| {
        match &state.state_type {
            SlackElementType::PlaneTextInput => {
                if state.name == "name" {
                    lunch_spot_name = state.value.clone()
                }
            }
            SlackElementType::Checkboxes => {
                if let Some(selected_options) = &state.selected_options {
                    selected_options.iter().for_each(|checked_box| {
                        open_days.insert(checked_box.value.clone(), true);
                    })
                }
            }
            _ => {}
        }
    });

    if let Some(name) = lunch_spot_name {
        Ok(LunchSpot::new(name,
                       open_days.contains_key(Weekday::Mon.to_string().as_str()),
                       open_days.contains_key(Weekday::Tue.to_string().as_str()),
                       open_days.contains_key(Weekday::Wed.to_string().as_str()),
                       open_days.contains_key(Weekday::Thu.to_string().as_str()),
                       open_days.contains_key(Weekday::Fri.to_string().as_str()),
        ))
    }
    else
    {
        Err(Box::new(ValidationError::new(&"How can we go to a lunch spot that doesn't have a name?".to_string())))
    }
}