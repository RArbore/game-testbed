/*
 * This file is part of game-testbed.
 * game-testbed is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * any later version.
 * game-testbed is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 * You should have received a copy of the GNU General Public License
 * along with game-testbed. If not, see <https://www.gnu.org/licenses/>.
 */

use winit::event::*;

const NUM_KEYCODES: usize = 164;

pub enum ControllerScheme {
    KeyboardMouse {
        jump_key: VirtualKeyCode,
        crouch_key: VirtualKeyCode,
        left_key: VirtualKeyCode,
        right_key: VirtualKeyCode,
    },
}

pub struct Controller {
    pressed: [bool; NUM_KEYCODES],
    scheme: ControllerScheme,
}

pub struct UserInput {
    jump: bool,
    crouch: bool,
    left: bool,
    right: bool,
}

impl Controller {
    pub fn new(scheme: ControllerScheme) -> Self {
        Controller {
            pressed: [false; NUM_KEYCODES],
            scheme,
        }
    }

    pub fn process_event(&mut self, event: &WindowEvent) -> bool {
        match (event, &self.scheme) {
            (
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state,
                            virtual_keycode: Some(keycode),
                            ..
                        },
                    ..
                },
                ControllerScheme::KeyboardMouse { .. },
            ) => {
                self.pressed[*keycode as usize] = *state == ElementState::Pressed;
                true
            }
            _ => false,
        }
    }

    pub fn get_user_input(&self) -> UserInput {
        match self.scheme {
            ControllerScheme::KeyboardMouse {
                jump_key,
                crouch_key,
                left_key,
                right_key,
            } => UserInput {
                jump: self.pressed[jump_key as usize],
                crouch: self.pressed[crouch_key as usize],
                left: self.pressed[left_key as usize],
                right: self.pressed[right_key as usize],
            },
        }
    }
}
