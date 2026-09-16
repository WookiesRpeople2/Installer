use crossterm::event::{KeyCode, KeyEvent};

use crate::{
    components::{InputComponent, prompt::Prompt, scroll_list::ScrollList},
    steps::Transition,
};

pub fn handle_on_key_menu<T, F, R>(t: &mut T, key: KeyEvent, func: F) -> Transition
where
    T: InputComponent,
    F: FnOnce(&mut T) -> R,
{
    match key.code {
        KeyCode::BackTab => Transition::Back,
        _ if t.on_key(key) => {
            func(t);
            Transition::Next
        }
        _ => Transition::None,
    }
}

pub fn handle_on_key_menu_prompt(prompts: &mut [&mut Prompt], key: KeyEvent) -> Transition {
    if !prompts.iter().any(|p| p.is_focused()) {
        if let Some(first) = prompts.iter_mut().next() {
            first.set_focused(true);
        }
    }

    match key.code {
        KeyCode::Tab | KeyCode::Down => {
            cycle_focus(prompts, true);
            Transition::None
        }
        KeyCode::Up => {
            cycle_focus(prompts, false);
            Transition::None
        }
        KeyCode::Enter if !all_prompts_filled(prompts) => {
            for p in prompts.iter_mut() {
                p.set_invalid(p.is_empty());
            }
            Transition::None
        }
        _ => match prompts.iter_mut().find(|p| p.is_focused()) {
            Some(active) => prompt(active, key),
            _ => Transition::None,
        },
    }
}

pub fn handle_on_key_menu_scroll_list(p: &mut ScrollList, key: KeyEvent) -> Transition {
    handle_on_key_menu(p, key, |_| ())
}

fn prompt(p: &mut Prompt, key: KeyEvent) -> Transition {
    handle_on_key_menu(p, key, |p| p.take_trimmed())
}

fn all_prompts_filled(prompts: &[&mut Prompt]) -> bool {
    prompts.iter().all(|p| !p.value().trim().is_empty())
}

fn cycle_focus(prompts: &mut [&mut Prompt], forward: bool) {
    let len = prompts.len();
    if len == 0 {
        return;
    }

    let current = prompts.iter().position(|p| p.is_focused()).unwrap_or(0);
    let next = if forward {
        (current + 1) % len
    } else {
        (current + len - 1) % len
    };

    for (i, p) in prompts.iter_mut().enumerate() {
        p.set_focused(i == next);
        if i == next {
            p.clear_invalid();
        }
    }
}
