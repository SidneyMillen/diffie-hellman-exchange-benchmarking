mod clients;
mod key_exchange;
use std::{f32::consts::PI, io::Read};

use macroquad::prelude::*;
use x25519_dalek::PublicKey;

use crate::clients::StaticKeyClient;

const KEY_TRANSFER_SHAPE_SIZE: f32 = 15.0;

//just cause this is really the value i need to use in the code anyway
const HALF_KEY_TRANSFER_SHAPE_SIZE: f32 = KEY_TRANSFER_SHAPE_SIZE / 2.0;

#[macroquad::main("Diffie Hellman")]
async fn main() {
    'outer: loop {
        let circle_size = 250.0;

        let num_clients: u8 = 10;

        let mut key_transfer_speed = 0.01;

        let mut game_clients: Vec<GameClient> = vec![];

        let mut key_transfers: Vec<PubkeyTransferVisualization> = vec![];

        let points = get_even_circle_points(circle_size, num_clients.into());

        let inner_points =
            get_even_circle_points(circle_size - circle_size * 0.20, num_clients.into());

        for (id, point) in points.iter().enumerate() {
            let game_client = GameClient::new(*point, id as u8);
            let next_point = points.get(id + 1).unwrap_or(points.get(0).unwrap());

            let key_transfer = PubkeyTransferVisualization {
                start_x: point.x,
                start_y: point.y,
                end_x: next_point.x,
                end_y: next_point.y,
                progress: 0.0,
                original_sender_id: id as u32,
                value: game_client.client.pubkey,
                remaining_hops: points.iter().len() as u8 - 1,
                color: pubkey_color_representation(&game_client.client.pubkey),
            };

            game_clients.push(game_client);
            key_transfers.push(key_transfer)
        }

        let mut process_finished = false;

        'inner: loop {
            if is_key_released(KeyCode::R) {
                next_frame().await;
                break 'inner;
            }

            if is_key_released(KeyCode::Q) {
                next_frame().await;
                break 'outer;
            }

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

            for transfer in key_transfers.iter_mut() {
                transfer.render(Vec2::new(center_x, center_y));
                //this is for the last part where the mutual secrets move into the center of the circle
                if process_finished && transfer.progress <= 1.0 {
                    transfer.progress += key_transfer_speed;
                }
            }
            if !process_finished {
                for mut transfer in key_transfers.iter_mut() {
                    transfer.progress += key_transfer_speed;

                    if transfer.progress >= 1.0 {
                        transfer.remaining_hops -= 1;
                        if transfer.remaining_hops == 0 {
                            process_finished = true;
                        }

                        let new_start_client_idx: usize = (transfer.original_sender_id as usize
                            + (num_clients as usize - 1 - transfer.remaining_hops as usize))
                            % num_clients as usize;
                        let new_end_client_idx: usize =
                            (new_start_client_idx + 1) % num_clients as usize;

                        let new_start_client =
                            game_clients.get(new_start_client_idx as usize).unwrap();
                        let mut new_start_point =
                            new_start_client.pos;
                        let new_end_client = game_clients.get(new_end_client_idx as usize).unwrap();
                        let mut new_end_point = new_end_client.pos;

                        let new_pubkey: PublicKey = new_start_client
                            .client
                            .calculate_shared_secret(&transfer.value)
                            .to_bytes()
                            .into();

                        transfer.update_pubkey(new_pubkey);

                        if process_finished{
                            new_end_point =
                                    inner_points.get(new_start_client_idx).unwrap().clone();
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

/// returns a vec of x and y values for n points distributed on a circle of a given size, with the first point at the top center of the circle
fn get_even_circle_points(circle_size: f32, num_points: isize) -> Vec<Vec2> {
    let angle_increment = 2.0 * PI / num_points as f32;

    let mut points = vec![];

    for i in 0..num_points {
        let angle = angle_increment * i as f32;
        let x = angle.sin() * circle_size;
        let y = angle.cos() * circle_size;

        points.push(Vec2::new(x, y))
    }

    return points;
}

///represents one of the participants in the exchange in the game
struct GameClient {
    client: StaticKeyClient,
    pos: Vec2,
    color: Color,
}

impl GameClient {
    fn new(pos: Vec2, id: u8) -> Self {
        let client = StaticKeyClient::new(id);
        let color = pubkey_color_representation(&client.pubkey);

        GameClient { client, pos, color }
    }
}

///basically just takes the first three bytes of the pubkey and converts it to a color
fn pubkey_color_representation(key: &PublicKey) -> Color {
    let key_bytes = key.to_bytes();
    let color_bytes = key_bytes.take(3).into_inner();

    //these unwraps are fine cause i took 3 bytes from the pubkey so there will be 3 u8s in the array
    let r = *color_bytes.get(0).unwrap();
    let g = *color_bytes.get(1).unwrap();
    let b = *color_bytes.get(2).unwrap();

    return Color::from_rgba(r, g, b, 255u8);
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
    pub remaining_hops: u8,
    color: Color,
}

impl PubkeyTransferVisualization {
    fn render(&self, center_point: Vec2) {
        //lerping hard or hardly lerping?
        let current_x = self.start_x + self.progress * (self.end_x - self.start_x);
        let current_y = (1.0 - self.progress) * self.start_y + self.progress * self.end_y;

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
