use yew::prelude::*;
use web_sys::{HtmlInputElement, HtmlAudioElement};
use wasm_bindgen_futures::JsFuture;
use gloo_timers::future::TimeoutFuture;
use js_sys::Math;

/// production/development aware static url
#[cfg(debug_assertions)]
macro_rules! static_audio_url {
    ($rest:tt) => {
        concat!("/audio/", $rest)
    };
}

#[cfg(not(debug_assertions))]
macro_rules! static_audio_url {
    ($rest:tt) => {
        concat!(
            "https://assets.siyuanyan.net/jiggle/audio/",
            $rest
        )
    };
}

use yew::TargetCast;
fn main() {
    set_event_bubbling(false);
    yew::start_app::<EFGTriplet>();
}


#[function_component(EFGTriplet)]
pub fn e_f_g_triplet() -> Html {
    let view_box_x = 300f32;
    let line_starting_y = 50f32;
    let note_head_rx = use_state_eq(|| 8f32);
    let note_head_ry = use_state_eq(|| 12f32);
    let note_slant_deg = use_state_eq(|| 65f32);
    let playing = use_state(|| false);
    let random_params = use_state(|| (8f32, 12f32, 65f32));
    if *playing {
        note_head_rx.set((*random_params).0);
        note_head_ry.set((*random_params).1);
        note_slant_deg.set((*random_params).2);
    };

    let height_of_rotated_ellipse = |a: f32, b: f32| {
        (((a.powi(2) + b.powi(2)) - ((a.powi(2) - b.powi(2)) * (note_slant_deg.to_radians() * 2f32).cos())) * 2f32).sqrt()
    };
    let line_spacing = height_of_rotated_ellipse(*note_head_rx, *note_head_ry);

    let note_group_starting_x = view_box_x / 7f32;


    let note_center_x = |n: u8| {
        note_group_starting_x + n as f32 * 59f32
    };
    let audio_ref = use_node_ref();


    use_ref({

        let audio_ref = audio_ref.clone();
        let random_params = random_params.clone();

        move ||{
        wasm_bindgen_futures::spawn_local(async move {
            let audio: HtmlAudioElement = audio_ref.cast().unwrap();
            loop{
                TimeoutFuture::new(100).await;
                if ! audio.paused(){
                    let note_head_rx = Math::random() * 1f64 + 7.5f64;
                    let note_head_ry = Math::random() * 1f64 + 11.5f64;
                    let note_slant_deg= Math::random() * 10f64 + 55f64;
                    random_params.set((note_head_rx as f32, note_head_ry as f32, note_slant_deg as f32));
                }


            }
        }
        )
    }});


    let play_audio = {
        let audio_ref = audio_ref.clone();
        let playing = playing.clone();
        let random_params = random_params.clone();
        move |_: MouseEvent| {
            let audio: HtmlAudioElement = audio_ref.cast().unwrap();
            let playing = playing.clone();
            let random_params = random_params.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if audio.paused() {
                    playing.set(true);
                    audio.set_current_time(0f64);
                    if let Ok(p)  = audio.play(){
                        JsFuture::from(p).await.unwrap();
                    }


                }else{
                    random_params.set((8f32, 12f32, 65f32));
                    audio.pause().ok();
                    playing.set(false);

                }

            });

        }};
    let note_no = {

        let note_slant_deg = note_slant_deg.clone();
        let note_head_rx = note_head_rx.clone();
        let note_head_ry = note_head_ry.clone();
        let play_audio = play_audio.clone();


        move |n: u8| {
        let first_line_y = line_starting_y + line_spacing * 4f32;

        let cx: f32 = note_center_x(n);
        let stalk_root_x = cx + *note_head_ry * note_slant_deg.to_radians().sin();
        let cy = first_line_y - n as f32 * line_spacing / 2f32;
        let stalk_root_y = cy - (stalk_root_x - cx) / note_slant_deg.to_radians().tan();
        let text_x = cx - *note_head_rx;


        let cx = cx.to_string();
        let cy = cy.to_string();
        let stalk_root_x = stalk_root_x.to_string();
        let text = match n {
            0 => "I",
            1 => "wish",
            2 => "you",
            _ => unreachable!(),
        };

        html! {
            <>
            <line style="stroke-width: 3;" x1={stalk_root_x.clone()} y1={stalk_root_y.to_string()} x2={stalk_root_x} y2={(stalk_root_y - 55f32).to_string()} onclick={play_audio.clone()}/>
            <ellipse rx={note_head_rx.to_string()} ry={note_head_ry.to_string()} cx={cx.clone()} cy={cy.clone()} transform={format!("rotate({} {cx} {cy})", *note_slant_deg)} onclick={play_audio.clone()}/>
            <text x={text_x.to_string()} y={(first_line_y + line_spacing * 1.5).to_string()} font-size=20 onclick={play_audio.clone()}>{text}</text>
            </>
        }
    }};



    let bracket_bottom_y = line_starting_y - line_spacing;
    let bracket_top_y = line_starting_y - line_spacing * 2f32;
    let bracket_left_x = note_group_starting_x - *note_head_rx * 1.5;
    let bracket_right_x = note_center_x(2) + *note_head_rx * 1.5;
    let bracket_mid_left_x = (bracket_left_x * 1.8f32 + bracket_right_x) / 3f32;
    let bracket_mid_right_x = (bracket_left_x + 1.8f32 * bracket_right_x) / 3f32;




    html! {
        <>
        <audio  ref={audio_ref} src={static_audio_url!("i-wish-you.mp3")} onended={move |_|{
            playing.set(false);
            random_params.set((8f32, 12f32, 65f32));
        }}/>
        <label>
            {"音符倾斜度: "}
            <input disabled={false} type="range" min=45 max=120 value={(*note_slant_deg).to_string()} onchange={move |e: Event|{
                let i: HtmlInputElement = e.target_unchecked_into();
                let value = i.value().parse::<f32>().unwrap();
                note_slant_deg.set(value);

        }}/>
        </label>
        <br/>
        <br/>
        <label>
            {"符头半径 a："}
            <input type="range" min=2 max=9 step=0.1 value={(*note_head_rx).to_string()} onchange={move |e: Event|{
                let i: HtmlInputElement = e.target_unchecked_into();
                let value = i.value().parse::<f32>().unwrap();
                note_head_rx.set(value);

        }}/>
        </label>
        <br/>
        <br/>
        <label>
            {"符头半径 b："}
            <input type="range" min=2 max=14 step=0.1 value={(*note_head_ry).to_string()} onchange={move |e: Event|{
                let i: HtmlInputElement = e.target_unchecked_into();
                let value = i.value().parse::<f32>().unwrap();
                note_head_ry.set(value);

        }}/>
        </label>
        <br/>
        <p>{"点击谱面播放 / click on the notes to play:"}</p>

    <svg viewBox={format!("0 0 {view_box_x} 170")} width="3em" style="font-size: 6rem; outline: 3px solid red; cursor:pointer;" onclick={play_audio.clone()}>


        <g style="fill: none; stroke: black; stroke-width:2px;">
            <polyline points={format!("{},{} {},{} {},{} ", bracket_left_x, bracket_bottom_y, bracket_left_x, bracket_top_y, bracket_mid_left_x, bracket_top_y)} onclick={play_audio.clone()}/>
            <text style="font-style: italic;" font-size=20 transform="translate(-5 7)" x={((bracket_left_x + bracket_right_x)/ 2f32).to_string()} y ={bracket_top_y.to_string()} onclick={play_audio.clone()}>{"3"}</text>
            <polyline points={format!("{},{} {},{} {},{}",  bracket_mid_right_x, bracket_top_y, bracket_right_x, bracket_top_y, bracket_right_x, bracket_bottom_y)} onclick={play_audio.clone()}/>
        </g>
      <g style="fill:black;stroke: black;">

        <g style="stroke-width:2;" >
                {for [0, 1, 2, 3, 4].into_iter().map(|x|{
                let y:String = (line_starting_y + (x as f32) * line_spacing).to_string();
                html!{
                    <line x1=0 y1={y.clone()} x2={view_box_x.to_string()} y2={y} onclick={play_audio.clone()}/>
                }
            })}
        </g>



        {for [0,1,2].into_iter().map(note_no)}
      </g>


    </svg>
        </>
    }
}