mod clients;


use std::{f32::consts::PI, io::Read};
use macroquad::{
    prelude::*,
    ui::{
        hash, root_ui,
        widgets::{self, Group},
    },
};
use x25519_dalek::PublicKey;

use crate::clients::StaticKeyClient;

const KEY_TRANSFER_SHAPE_SIZE: f32 = 15.0;

//might as well precalculate this
const HALF_KEY_TRANSFER_SHAPE_SIZE: f32 = KEY_TRANSFER_SHAPE_SIZE / 2.0;

#[macroquad::main("Diffie Hellman")]
async fn main() {
    let mut num_clients: u16 = 25;

    let mut key_transfer_speed = 2.0;

    loop {
        let mut new_num_clients = num_clients as f32;

        let circle_size = 250.0;

        let mut game_clients: Vec<GameClient> = vec![];

        let mut key_transfers: Vec<PubkeyTransferVisualization> = vec![];

        let points = get_even_circle_points(circle_size, num_clients as isize);

        let inner_circle_points =
            get_even_circle_points(circle_size - circle_size * 0.20, num_clients as isize);

        for (id, point) in points.iter().enumerate() {
            let game_client = GameClient::new(*point, id as u16);
            let next_point = points.get(id + 1).unwrap_or(points.first().unwrap());

            let key_transfer = PubkeyTransferVisualization {
                start_x: point.x,
                start_y: point.y,
                end_x: next_point.x,
                end_y: next_point.y,
                progress: 0.0,
                original_sender_id: id as u32,
                value: game_client.client.pubkey,
                remaining_hops: points.iter().len() as u16 - 1,
                color: pubkey_color_representation(&game_client.client.pubkey),
            };

            game_clients.push(game_client);
            key_transfers.push(key_transfer)
        }

        let mut process_finished = false;

        'inner: loop {
            widgets::Window::new(hash!(), vec2(0., 0.), vec2(500.0, 150.0))
                .label("Settings")
                .titlebar(true)
                .ui(&mut root_ui(), |ui| {
                    ui.label(None, "Press \"R\" to reset");
                    ui.label(None, "Clients:");
                    ui.slider(hash!(), "[2 - 255]", 2f32..255f32, &mut new_num_clients);
                    ui.label(None, "Transfers/sec:");
                    ui.slider(
                        hash!(),
                        "[0.1 - 10.0]",
                        0.1f32..10f32,
                        &mut key_transfer_speed,
                    );
                });

            new_num_clients = new_num_clients.round();

            if new_num_clients as u16 != num_clients && is_mouse_button_released(MouseButton::Left) {
                //wait to reset game till user is done moving slider
                num_clients = new_num_clients as u16;
                break 'inner;
            }

            if is_key_released(KeyCode::R) {
                next_frame().await;
                break 'inner;
            }

            // if is_key_released(KeyCode::Q) {
            //     next_frame().await;
            //     break 'outer;
            // }

            let center_x = screen_width() / 2.0;
            let center_y = screen_height() / 2.0;

            clear_background(WHITE);

            draw_circle_lines(center_x, center_y, circle_size, 1.0, GRAY);

            for client in game_clients.iter() {
                draw_circle(
                    center_x + client.pos.x,
                    center_y + client.pos.y,
                    10.0,
                    client.color,
                );
            }

            if process_finished {
                let shared_secret = key_transfers.first().unwrap().value.to_bytes();
                let secret_text = &shared_secret
                    .iter()
                    .map(|byte| byte.to_string())
                    .collect::<Vec<_>>()
                    .join("");
                let display_text = format!("Shared Secret: {}", secret_text);
                let font_size = 15;

                let text_center = get_text_center(&display_text, None, font_size, 1.0, 0.0);
                draw_text(
                    &display_text,
                    center_x - text_center.x,
                    center_y - text_center.y,
                    font_size as f32,
                    BLACK,
                );
            }

            for transfer in key_transfers.iter_mut() {
                transfer.render(Vec2::new(center_x, center_y));
                //this is for the last part where the mutual secrets move into the center of the circle
                if process_finished && transfer.progress <= 1.0 {
                    transfer.progress += key_transfer_speed * get_frame_time();
                }
            }
            if !process_finished {
                for transfer in key_transfers.iter_mut() {
                    transfer.progress += key_transfer_speed * get_frame_time();

                    if transfer.progress >= 1.0 {
                        transfer.remaining_hops -= 1;
                        if transfer.remaining_hops == 0 {
                            process_finished = true;
                        }

                        let new_start_client_idx: u16 = (transfer.original_sender_id as u16
                            + (num_clients - 1 - transfer.remaining_hops))
                            % num_clients;
                        let new_end_client_idx: u16 = (new_start_client_idx + 1) % num_clients;

                        let new_start_client =
                            game_clients.get(new_start_client_idx as usize).unwrap();
                        let new_start_point = new_start_client.pos;
                        let new_end_client = game_clients.get(new_end_client_idx as usize).unwrap();
                        let mut new_end_point = new_end_client.pos;

                        let new_pubkey: PublicKey = new_start_client
                            .client
                            .calculate_shared_secret(&transfer.value)
                            .to_bytes()
                            .into();

                        transfer.update_pubkey(new_pubkey);

                        if process_finished {
                            new_end_point = *inner_circle_points
                                .get(new_start_client_idx as usize)
                                .unwrap();
                        }

                        transfer.end_x = new_end_point.x;
                        transfer.end_y = new_end_point.y;

                        transfer.start_x = new_start_point.x;
                        transfer.start_y = new_start_point.y;

                        transfer.progress = 0.0;
                    }
                }
            }

            next_frame().await
        }
    }
}

/// returns a vec of x and y values for n points distributed on a circle of a given size, with the first point at the bottom center of the circle
fn get_even_circle_points(circle_size: f32, num_points: isize) -> Vec<Vec2> {
    let angle_increment = 2.0 * PI / num_points as f32;

    let mut points = vec![];

    for i in 0..num_points {
        let angle = angle_increment * i as f32;
        let x = angle.sin() * circle_size;
        let y = angle.cos() * circle_size;

        points.push(Vec2::new(x, y))
    }

    points
}

///represents one of the participants in the exchange in the game
struct GameClient {
    client: StaticKeyClient,
    pos: Vec2,
    color: Color,
}

impl GameClient {
    fn new(pos: Vec2, id: u16) -> Self {
        let client = StaticKeyClient::new(id);
        let color = pubkey_color_representation(&client.pubkey);

        GameClient { client, pos, color }
    }
}

///basically just takes the first three bytes of the pubkey and converts it to a color
fn pubkey_color_representation(key: &PublicKey) -> Color {
    let key_bytes = key.to_bytes();
    let color_bytes = key_bytes.take(3).into_inner();

    //these unwraps are fine cause i took 3 bytes from the pubkey so there will be 3 u16s in the array
    let r = *color_bytes.first().unwrap();
    let g = *color_bytes.get(1).unwrap();
    let b = *color_bytes.get(2).unwrap();

    Color::from_rgba(r, g, b, 255u8)
}

struct PubkeyTransferVisualization {
    pub start_x: f32,
    pub start_y: f32,
    pub end_x: f32,
    pub end_y: f32,
    ///will be 0 when at the start of the transfer and 1.0 at the end
    pub progress: f32,
    ///the client id of the original sender used to determine when the negotiation is finished
    pub original_sender_id: u32,
    pub value: PublicKey,
    pub remaining_hops: u16,
    color: Color,
}

impl PubkeyTransferVisualization {
    fn render(&self, center_point: Vec2) {
        //lerping hard or hardly lerping?
        let current_x = self.start_x + self.progress * (self.end_x - self.start_x);
        let current_y = self.start_y + self.progress * (self.end_y - self.start_y);

        draw_rectangle(
            center_point.x + current_x - HALF_KEY_TRANSFER_SHAPE_SIZE,
            center_point.y + current_y - HALF_KEY_TRANSFER_SHAPE_SIZE,
            KEY_TRANSFER_SHAPE_SIZE,
            KEY_TRANSFER_SHAPE_SIZE,
            self.color,
        )
    }
    fn update_pubkey(&mut self, new_pubkey: PublicKey) {
        self.color = pubkey_color_representation(&new_pubkey);
        self.value = new_pubkey;
    }
}
