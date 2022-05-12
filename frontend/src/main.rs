use yew_hooks::prelude::*;
use derive_more::Deref;
use futures::stream::{SplitSink, SplitStream};
use futures::{SinkExt, StreamExt};
use gloo_timers::callback::Timeout;
use gloo_timers::future::TimeoutFuture;
use reqwasm::websocket::futures::WebSocket;
use reqwasm::websocket::Message;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::{BinaryHeap, BTreeSet};
use std::f64;
use std::ops::Deref;

use std::rc::Rc;
use std::sync::Mutex;
use std::sync::RwLock;
use chinese_number::{ChineseNumber, ChineseVariant};
use stylist::Style;
use time::macros::date;

use stylist::yew::styled_component;
use stylist::yew::use_style;


use web_sys::{CssStyleDeclaration, Element, HtmlDivElement, HtmlHeadingElement};
use yew::prelude::*;

use yew_vdom_gen::prelude::*;
use yewdux::prelude::*;
use yewdux_functional::*;
use stylist::yew::Global;
use time::{Date, Month};
use yew::virtual_dom::{AttrValue, VText};
use bounce::{Atom, BounceRoot, use_atom_setter, use_atom_value, use_atom};
use bounce::prelude::*;
use js_sys::JsString;
use wasm_bindgen::{JsCast, JsValue};

// macro_rules! use_style {
//     ($a: tt) => {{
//         let style = stylist::yew::use_style!($a);
//         let style = style.get_class_name().to_owned();
//         let attr_val: AttrValue = style.into();
//         attr_val
//     }};
// }


/// production/development aware static url
#[cfg(debug_assertions)]
macro_rules! static_url {
    ($rest:tt) => {
        concat!("/image/", $rest)
    };
}

#[cfg(not(debug_assertions))]
macro_rules! static_url {
    ($rest:tt) => {
        concat!(
            "https://assets.siyuanyan.net/what-now/image/",
            $rest
        )
    };
}

#[derive(PartialEq, Clone, Copy)]
enum Hour {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    Thirteen,
    Fourteen,
    Fifteen,
    Sixteen,
    Seventeen,
    Eighteen,
    Nineteen,
    Twenty,
    TwentyOne,
    TwentyTwo,
    TwentyThree,
}

impl From<Hour> for u8 {
    fn from(hour: Hour) -> Self {
        match hour {
            Hour::Zero => 0,
            Hour::One => 1,
            Hour::Two => 2,
            Hour::Three => 3,
            Hour::Four => 4,
            Hour::Five => 5,
            Hour::Six => 6,
            Hour::Seven => 7,
            Hour::Eight => 8,
            Hour::Nine => 9,
            Hour::Ten => 10,
            Hour::Eleven => 11,
            Hour::Twelve => 12,
            Hour::Thirteen => 13,
            Hour::Fourteen => 14,
            Hour::Fifteen => 15,
            Hour::Sixteen => 16,
            Hour::Seventeen => 17,
            Hour::Eighteen => 18,
            Hour::Nineteen => 19,
            Hour::Twenty => 20,
            Hour::TwentyOne => 21,
            Hour::TwentyTwo => 22,
            Hour::TwentyThree => 23,
        }
    }
}

impl From<u8> for Hour {
    fn from(hour: u8) -> Self {
        match hour {
            0 => Hour::Zero,
            1 => Hour::One,
            2 => Hour::Two,
            3 => Hour::Three,
            4 => Hour::Four,
            5 => Hour::Five,
            6 => Hour::Six,
            7 => Hour::Seven,
            8 => Hour::Eight,
            9 => Hour::Nine,
            10 => Hour::Ten,
            11 => Hour::Eleven,
            12 => Hour::Twelve,
            13 => Hour::Thirteen,
            14 => Hour::Fourteen,
            15 => Hour::Fifteen,
            16 => Hour::Sixteen,
            17 => Hour::Seventeen,
            18 => Hour::Eighteen,
            19 => Hour::Nineteen,
            20 => Hour::Twenty,
            21 => Hour::TwentyOne,
            22 => Hour::TwentyTwo,
            23 => Hour::TwentyThree,
            _ => panic!("invalid hour")
        }
    }
}


#[derive(PartialEq, Clone, Copy)]
enum Minute {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    Thirteen,
    Fourteen,
    Fifteen,
    Sixteen,
    Seventeen,
    Eighteen,
    Nineteen,
    Twenty,
    TwentyOne,
    TwentyTwo,
    TwentyThree,
    TwentyFour,
    TwentyFive,
    TwentySix,
    TwentySeven,
    TwentyEight,
    TwentyNine,
    Thirty,
    ThirtyOne,
    ThirtyTwo,
    ThirtyThree,
    ThirtyFour,
    ThirtyFive,
    ThirtySix,
    ThirtySeven,
    ThirtyEight,
    ThirtyNine,
    Forty,
    FortyOne,
    FortyTwo,
    FortyThree,
    FortyFour,
    FortyFive,
    FortySix,
    FortySeven,
    FortyEight,
    FortyNine,
    Fifty,
    FiftyOne,
    FiftyTwo,
    FiftyThree,
    FiftyFour,
    FiftyFive,
    FiftySix,
    FiftySeven,
    FiftyEight,
    FiftyNine,
}

impl From<Minute> for u8 {
    fn from(minute: Minute) -> Self {
        match minute {
            Minute::Zero => 0,
            Minute::One => 1,
            Minute::Two => 2,
            Minute::Three => 3,
            Minute::Four => 4,
            Minute::Five => 5,
            Minute::Six => 6,
            Minute::Seven => 7,
            Minute::Eight => 8,
            Minute::Nine => 9,
            Minute::Ten => 10,
            Minute::Eleven => 11,
            Minute::Twelve => 12,
            Minute::Thirteen => 13,
            Minute::Fourteen => 14,
            Minute::Fifteen => 15,
            Minute::Sixteen => 16,
            Minute::Seventeen => 17,
            Minute::Eighteen => 18,
            Minute::Nineteen => 19,
            Minute::Twenty => 20,
            Minute::TwentyOne => 21,
            Minute::TwentyTwo => 22,
            Minute::TwentyThree => 23,
            Minute::TwentyFour => 24,
            Minute::TwentyFive => 25,
            Minute::TwentySix => 26,
            Minute::TwentySeven => 27,
            Minute::TwentyEight => 28,
            Minute::TwentyNine => 29,
            Minute::Thirty => 30,
            Minute::ThirtyOne => 31,
            Minute::ThirtyTwo => 32,
            Minute::ThirtyThree => 33,
            Minute::ThirtyFour => 34,
            Minute::ThirtyFive => 35,
            Minute::ThirtySix => 36,
            Minute::ThirtySeven => 37,
            Minute::ThirtyEight => 38,
            Minute::ThirtyNine => 39,
            Minute::Forty => 40,
            Minute::FortyOne => 41,
            Minute::FortyTwo => 42,
            Minute::FortyThree => 43,
            Minute::FortyFour => 44,
            Minute::FortyFive => 45,
            Minute::FortySix => 46,
            Minute::FortySeven => 47,
            Minute::FortyEight => 48,
            Minute::FortyNine => 49,
            Minute::Fifty => 50,
            Minute::FiftyOne => 51,
            Minute::FiftyTwo => 52,
            Minute::FiftyThree => 53,
            Minute::FiftyFour => 54,
            Minute::FiftyFive => 55,
            Minute::FiftySix => 56,
            Minute::FiftySeven => 57,
            Minute::FiftyEight => 58,
            Minute::FiftyNine => 59,
        }
    }
}

impl From<u8> for Minute {
    fn from(minute: u8) -> Self {
        match minute {
            0 => Minute::Zero,
            1 => Minute::One,
            2 => Minute::Two,
            3 => Minute::Three,
            4 => Minute::Four,
            5 => Minute::Five,
            6 => Minute::Six,
            7 => Minute::Seven,
            8 => Minute::Eight,
            9 => Minute::Nine,
            10 => Minute::Ten,
            11 => Minute::Eleven,
            12 => Minute::Twelve,
            13 => Minute::Thirteen,
            14 => Minute::Fourteen,
            15 => Minute::Fifteen,
            16 => Minute::Sixteen,
            17 => Minute::Seventeen,
            18 => Minute::Eighteen,
            19 => Minute::Nineteen,
            20 => Minute::Twenty,
            21 => Minute::TwentyOne,
            22 => Minute::TwentyTwo,
            23 => Minute::TwentyThree,
            24 => Minute::TwentyFour,
            25 => Minute::TwentyFive,
            26 => Minute::TwentySix,
            27 => Minute::TwentySeven,
            28 => Minute::TwentyEight,
            29 => Minute::TwentyNine,
            30 => Minute::Thirty,
            31 => Minute::ThirtyOne,
            32 => Minute::ThirtyTwo,
            33 => Minute::ThirtyThree,
            34 => Minute::ThirtyFour,
            35 => Minute::ThirtyFive,
            36 => Minute::ThirtySix,
            37 => Minute::ThirtySeven,
            38 => Minute::ThirtyEight,
            39 => Minute::ThirtyNine,
            40 => Minute::Forty,
            41 => Minute::FortyOne,
            42 => Minute::FortyTwo,
            43 => Minute::FortyThree,
            44 => Minute::FortyFour,
            45 => Minute::FortyFive,
            46 => Minute::FortySix,
            47 => Minute::FortySeven,
            48 => Minute::FortyEight,
            49 => Minute::FortyNine,
            50 => Minute::Fifty,
            51 => Minute::FiftyOne,
            52 => Minute::FiftyTwo,
            53 => Minute::FiftyThree,
            54 => Minute::FiftyFour,
            55 => Minute::FiftyFive,
            56 => Minute::FiftySix,
            57 => Minute::FiftySeven,
            58 => Minute::FiftyEight,
            59 => Minute::FiftyNine,
            _ => panic!("invalid minute"),
        }
    }
}

type Ss = &'static str;


#[derive(Properties, PartialEq, Clone)]
pub struct DayContainerProps {
    day_heading: ChildrenWithProps<DayHeading>,
    children: Children,
}

#[function_component(DayContainer)]
pub fn day_container(props: &DayContainerProps) -> Html {
    html! {
        <>
            { props.day_heading.clone() }
            { for props.children.clone() }
        </>
    }
}


#[styled_component(StickyTopBar)]
pub fn sticky_top_bar() -> Html {
    let date = use_slice_value::<UpThereDates>();
    // #[cfg(debug_assertions)]
    // log::info!("got sticks {:?}", date);


    let date = date.get_the_largest_date();


    date.map(|date| {
        html! {
            <div class={css!(r"
                    position: fixed;
                    top: 0;
                    background: white;
                    left: 0;
                    right: 0;
                    border-bottom: 2px solid black;
                ")}
            >
                <h2 class={css!(r"
                    margin-top: 0.83em;
                    margin-left: 8px;

                ")
                }>
                    { date.into_chinese_month_and_day()}
                </h2>
            </div>

    }
    })
        .unwrap_or_default()
}

macro_rules! day_heading {
    ($y:literal - $m:literal - $d:literal) => {
        {
            html!{
                <DayHeading date={Date::from_calendar_date($y, ($m as u8).try_into().unwrap(), $d).unwrap()}/>
            }
        }
    };
}



impl UpThereDates {
    pub fn get_the_largest_date(&self) -> Option<Date> {

        let large_date = self.0.iter().next_back();

        large_date.cloned()
    }
}


#[derive(Deref, Default, Slice)]
struct UpThereDates(BTreeSet<Date>);


impl PartialEq for UpThereDates {
    fn eq(&self, other: &Self) -> bool {
        self.len().eq(&other.len())
    }
}


enum StickDateUpdate {
    AddDateUpThere(Date),
    RemoveDateDownBelow(Date),
}


impl Reducible for UpThereDates {
    type Action = StickDateUpdate;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            StickDateUpdate::AddDateUpThere(d) => {
                if ! self.contains(&d){
                    let mut new_self = (*self).clone();
                    new_self.insert(d);
                    Rc::new(UpThereDates(new_self))
                }else{
                    self
                }
            }
            StickDateUpdate::RemoveDateDownBelow(d) => {
                if self.contains(&d){
                    let mut new_self = (*self).clone();
                    new_self.remove(&d);
                    Rc::new(UpThereDates(new_self))

                }else{
                    self
                }
            }
        }
    }
}


#[styled_component(App)]
pub fn app() -> Html {
    let enlarged_url: UseStateHandle<Option<&'static str>> = use_state(|| None);

    let show_enlarged = {
        let enlarged_url = enlarged_url.clone();

        Callback::from(move |url: &'static str| {
            enlarged_url.set(Some(url));
        })
    };


    let modal = css!(r"
    position: fixed;
    z-index: 1;
    left: 0;
    top: 0;
    width: 100%;
    height: 100%;
    overflow: auto;
    background-color: rgba(0,0,0,0.8);
    cursor: pointer;

    >img{
        max-width: 100%;
        position: absolute;
        top: 15%;
        left: 50%;
        transform: translateX(-50%);
    }
    ");


    let global = css!(r"

      body{
        max-width: 60rem;

      }

    ");


    let dismiss_modal = {
        let enlarged_url = enlarged_url.clone();
        Callback::from(move |_| {
            enlarged_url.set(None);
        })
    };


    let sing_myself_to_ya = { html_nested!(<a rel="noopener" target="_blank" href="https://youtu.be/nJ_OxtmY0Fc?t=36">{"想把我唱给你听"}</a>) };
    let i_wish_you_shelter = { html_nested!(<a rel="noopener" target="_blank" href="https://youtu.be/P1V6cQJpbc4?t=53">{"I wish you shelter ..."}</a>) };
    let i_wish_you_love = { html_nested!(<a rel="noopener" target="_blank" href="https://www.youtube.com/watch?v=issStxOM5kw">{"I wish you love"}</a>) };

    html! {
        <BounceRoot>
            <ContextProvider<Callback<Ss>> context={show_enlarged}>
                <Global css={global}/>

            if let Some(url) = *enlarged_url {
                <div class={modal} onclick={dismiss_modal.clone()}>
                  <img src={url} onclick={dismiss_modal}/>
                </div>
            }

                <p>{"俺的手机上有个程序，以随机时间点提醒俺记录当下，诚实，不添油加醋，没有 context"}</p>
                <p>{"希望这样做能给生活带来冷静、客观、理智、和觉察～"}</p>

                <StickyTopBar/>

                <YearHeading year=2022/>

                {day_heading!(2022-5-4)}

                <Entry<Ss> hour={Hour::Seventeen} minute={Minute::Twelve} expressor="reddit 上的一条评论说，\"叫爸爸\"" />
                <Entry<Ss> hour={Hour::Seventeen} minute={Minute::Eighteen} expressor="自言自语 \"waku waku\"，沈雨荷爱说这个，海贼王片头曲里也有" />
                <Entry<Ss> hour={Hour::Seventeen} minute={Minute::TwentySeven} expressor="想去万达找乐器店打架子鼓，左手里握着菜刀" />
                <Entry<Ss> hour={Hour::Seventeen} minute={Minute::Thirty} expressor="洗切菜板，想要不要在这个网站上偶尔对一些条目补充细节，又觉不妥，因为容易添油加醋，不是每刻真实所感。" />
                <Entry<Ss> hour={Hour::Seventeen} minute={Minute::ThirtyThree} expressor="刚写完上条又来一条，我去把时间调长一点" />
                <Entry<Ss> hour={Hour::Seventeen} minute={Minute::Forty} expressor="在和沈雨荷聊天" />
                <Entry<Ss> hour={Hour::Seventeen} minute={Minute::FiftyFive} expressor="跟沈雨荷说：\"我爸爸刚刚给我打电话\"" />
                <Entry<Ss> hour={Hour::Eighteen} minute={Minute::Five} expressor="沈雨荷说妹妹刚到英国的时候比现在瘦" />
                <Entry<Ss> hour={Hour::Eighteen} minute={Minute::ThirtySix} expressor="亲眼看到多了一条 log，心中一紧，手机紧接着响起通知" />
                <Entry<Ss> hour={Hour::Nineteen} minute={Minute::ThirtyFive} expressor="在盯着公路上驶来的车流，期盼空的出租车" />
                <Entry<Ss> hour={Hour::Twenty} minute={Minute::Five} expressor="在想之前去太原工作一开始隐瞒了自己是留学生，不想和老板产生距离感，可能反而是不真诚，还是不要这么做为好" />
                <Entry<Ss> hour={Hour::Twenty} minute={Minute::Forty} expressor="正在打开微信星巴克小程序，想喝甜甜的" />
                <Entry<TextAndFig<Ss, NoCaptionFigRenderer>>
                    hour={Hour::Twenty}
                    minute={Minute::FiftyFive}
                    expressor={
                        TextAndFig{
                            text: "在看reddit的这个帖子，尝试想象 “played this religiously in high school” 是什么样子",
                            src:static_url!("military-starter-pack.jpeg"),
                            fig_renderer: {NoCaptionFigRenderer}}}
                />
                <Entry<Ss> hour={Hour::TwentyOne} minute={Minute::TwentyNine} expressor="在逛 r/china_irl，心情抑郁，眉头微皱，紧抿双唇" />
                <Entry<TextAndFig<Ss, NoCaptionFigRenderer>> hour={Hour::TwentyOne} minute={Minute::TwentyTwo} expressor={TextAndFig{text: "心不在焉，看到朋友圈这张图上的日文，想象这条石头质感的鱼似乎可以和日本寺庙之类的景色很搭", src:static_url!("stone-fish.jpeg"), fig_renderer: {NoCaptionFigRenderer}}} />
                <Entry<Ss> hour={Hour::TwentyTwo} minute={Minute::Seventeen} expressor="在上衣口袋里寻找纸巾" />
                <Entry<Ss> hour={Hour::TwentyThree} minute={Minute::Six} expressor="走在回家路上昏昏沉沉，想到清醒是否比昏沉好？“注意力是一种原始的智慧” 忘记是哪听来的了，好像是 Eckhart Tolle？想到经文里写的 （大意）“走路的时候应当知道自己在走路，行住坐卧应当知道自己在行住坐卧”，感觉是一种注意力的锻炼。" />

                {day_heading!(2022-5-5)}

                <Entry<Ss> hour={Hour::Fifteen} minute={Minute::TwentyOne} expressor="找口罩" />
                <Entry<Ss> hour={Hour::Fifteen} minute={Minute::FiftyEight} expressor="在和😎聊天" />
                <Entry<Ss> hour={Hour::Sixteen} minute={Minute::Thirty} expressor="“把土豆泥做成冰淇淋给小孩子吃”" />
                <Entry<Ss> hour={Hour::Sixteen} minute={Minute::Eighteen} expressor="“那不是做这种事情的正常操作吗”" />
                <Entry<Ss> hour={Hour::Sixteen} minute={Minute::TwentySix} expressor="忘了（可能当时聊得太投入）" />
                <Entry<Ss> hour={Hour::Sixteen} minute={Minute::FiftyFour} expressor="“物理翻墙”" />
                <Entry<Ss> hour={Hour::Seventeen} minute={Minute::Fourteen} expressor="嚼劲爆鸡米花" />
                <Entry<Ss> hour={Hour::Seventeen} minute={Minute::TwentyEight} expressor="😎竟然知道恶臭数字梗" />
                <Entry<Ss> hour={Hour::Seventeen} minute={Minute::FortySeven} expressor="在想：会不会去万达的公交站台在修路的地段呢？" />
                <Entry<Html> hour={Hour::Eighteen} minute={Minute::Twelve} expressor={{html!{<> {"小声唱 "}{sing_myself_to_ya}</>}}} />
                <Entry<Ss> hour={Hour::Eighteen} minute={Minute::TwentySeven} expressor="右手握手机，微烫" />
                <Entry<Ss> hour={Hour::Eighteen} minute={Minute::ThirtySix} expressor="在怀疑是否真的有人会喜欢看“右手握手机，微烫”这种流水账" />
                <Entry<Ss> hour={Hour::Nineteen} minute={Minute::Seven} expressor="看着繁华街景，寻找那家乐器店" />
                <Entry<Ss> hour={Hour::Nineteen} minute={Minute::TwentyTwo} expressor="想象着小孩子因为鼓很帅气而想学鼓" />
                <Entry<Ss> hour={Hour::Nineteen} minute={Minute::FiftyNine} expressor="地图上有河，想象河的水汽清凉" />
                <Entry<Ss> hour={Hour::Twenty} minute={Minute::TwentyTwo} expressor="typing \"bruh I really wish I could get the position\"" />
                <Entry<Ss> hour={Hour::Twenty} minute={Minute::TwentySeven} expressor="弹幕说“43秒的视频怎么放出51秒的语音”" />
                <Entry<Ss> hour={Hour::Twenty} minute={Minute::FortyFive} expressor="按音量键放大音量，r/china_irl 的一个视频帖标题是“上海市民放国际歌被上门调查”" />
                <Entry<Ss> hour={Hour::TwentyOne} minute={Minute::FiftyOne} expressor="刚给沈雨荷发语音，日语的“肚子饿了”，她问我“这是方言吗”" />
                <Entry<TextAndFig<Html, NoCaptionFigRenderer>>
                    hour={Hour::TwentyOne}
                    minute={Minute::Eleven}
                    expressor={
                        TextAndFig{
                            text: html!{<> {"有人发猫猫图，说“born to loaf”。我在哼歌 "}{i_wish_you_shelter}</>},
                            src:static_url!("born-to-loaf.jpg"),
                            fig_renderer: {NoCaptionFigRenderer}}}
                />
                <Entry<TextAndFig<Html, NoCaptionFigRenderer>>
                    hour={Hour::TwentyOne}
                    minute={Minute::TwentyFive}
                    expressor={
                        TextAndFig{
                            text: html!{<> {"哼歌，还是 "}{i_wish_you_love}{"，解冻鸡汤块，指尖冰凉"}</>},
                            src:static_url!("chicken-soup.jpeg"),
                            fig_renderer: {NoCaptionFigRenderer}}}
                />
                <Entry<TextAndFig<Ss, NoCaptionFigRenderer>>
                    hour={Hour::TwentyOne}
                    minute={Minute::FiftySeven}
                    expressor={
                        TextAndFig{
                            text: "筷子在碗里叮当响，嘴里有滑溜的鸡汤面",
                            src:static_url!("chicken-noodles.jpeg"),
                            fig_renderer: {NoCaptionFigRenderer}
                        }
                    }
                />
                <Entry<TextAndFig<Ss, CaptionedFigRenderer>>
                    hour={Hour::TwentyTwo}
                    minute={Minute::TwentySix}
                    expressor={
                        TextAndFig{
                            text: "左手扶眼镜，自言自语，\"啊你的头像...waku waku\"",
                            src:static_url!("wakuwaku.jpeg"),
                            fig_renderer: {CaptionedFigRenderer{caption: "朋友的微信头像"}}
                        }
                    }
                />
                <Entry<Ss> hour={Hour::TwentyTwo} minute={Minute::ThirtyOne} expressor="在欣赏自己 repo 的 readme" />
                <Entry<Ss> hour={Hour::TwentyTwo} minute={Minute::FiftyFive} expressor="手指在触摸屏上滑动，想“这是 Zero 的第几课时口语作业呢”" />

                {day_heading!(2022-5-6)}

                <Entry<Ss> hour={Hour::Zero} minute={Minute::TwentyThree} expressor="在想 Adam Neily 用的一款神奇的 DAW 软件能够分析人声音调画出折线图" />
                <Entry<Ss> hour={Hour::Zero} minute={Minute::FortyEight} expressor="打字中，写到 \"...is used\"" />
                <Entry<Ss> hour={Hour::One} minute={Minute::Sixteen} expressor="(audacity 的文档) 读到 \"drag vertically\"" />
                <Entry<Ss> hour={Hour::One} minute={Minute::ThirtySix} expressor="我想说 ”我们那边的火警都是一起响的“" />
                <Entry<Ss> hour={Hour::Two} minute={Minute::Six} expressor="跟 iory 说，星战系列过誉了，只是经典而已" />
                <Entry<Ss> hour={Hour::Two} minute={Minute::ThirtyThree} expressor="搜索 U of A 的 subreddit，查找 Graduation GPA 的关键词" />
                <Entry<Ss> hour={Hour::Three} minute={Minute::Twelve} expressor="\"duh duh duh...\" （和 iory 语音，不知道要说什么的时候发出的 filler 声音）" />
                <Entry<Ss> hour={Hour::Three} minute={Minute::ThirtyFive} expressor="\"holy shit fft window size 调大了之后 frenquency resolution 高出好多\"" />
                <Entry<Ss> hour={Hour::Four} minute={Minute::ThirtySix} expressor="我低一个八度唱，然后 transpose，但这样就不真了" />
                <Entry<Ss> hour={Hour::Five} minute={Minute::Ten} expressor="搞黄色" />
                <Entry<Ss> hour={Hour::Five} minute={Minute::TwentyFour} expressor="和沈雨荷打字聊天" />
                <Entry<TextAndFig<Ss, NoCaptionFigRenderer>>
                    hour={Hour::Six}
                    minute={Minute::Three}
                    expressor={
                        TextAndFig{
                            text: "看了 r/china_irl 的这个帖子之后，感到压抑",
                            src:static_url!("vpn-trap.jpeg"),
                            fig_renderer: {NoCaptionFigRenderer}
                        }
                    }
                />
                <Entry<Ss> hour={Hour::Nine} minute={Minute::FiftyFour} expressor="想跟妈说：“你们自己吃吧我不知道做什么”" />
                <Entry<Ss> hour={Hour::Ten} minute={Minute::TwentyThree} expressor="在想 『搞黄色』 三个字是不是太缺少细节" />
                <Entry<Ss> hour={Hour::Ten} minute={Minute::ThirtySeven} expressor="（指 web3) 和书鱼打字：国内是接轨不了了" />
                <Entry<Ss> hour={Hour::Eleven} minute={Minute::Twelve} expressor="跟 iory 表示自己已经很累了：“but I've been awake for ... hours”" />
                <Entry<TextAndFig<Ss, NoCaptionFigRenderer>>
                    hour={Hour::Eleven}
                    minute={Minute::ThirtyFive}
                    expressor={
                        TextAndFig{
                            text: "bilibili 上的视频推荐",
                            src:static_url!("minecraft-bilibili-feed.jpeg"),
                            fig_renderer: {NoCaptionFigRenderer}
                        }
                    }
                />
                <Entry<Ss> hour={Hour::Eleven} minute={Minute::FortyThree} expressor="读自己给 kenny 发的消息 （解释为什么想找个架子鼓教室打工）\"I just want an excuse to touch the drums\" 右手在来回抚摸粗长的充电线因为好像有麻麻的感觉想验证是不是错觉" />
                <Entry<Ss> hour={Hour::Twelve} minute={Minute::Seven} expressor="在哼唱 “想把我唱给你听” 的前三个音" />
                <Entry<Ss> hour={Hour::Twelve} minute={Minute::Nine} expressor="（讨论英国的学校申请会不会全拒）我说 “还是有可能的”" />
                <Entry<Ss> hour={Hour::Thirteen} minute={Minute::ThirtyEight} expressor="想象 intellij 的程序员将 IDE 功能和具体框架高度整合，例如 Pycharm 和 Django 之类的" />
                <Entry<TextAndFig<Ss, CaptionedFigRenderer>>
                    hour={Hour::TwentyTwo}
                    minute={Minute::TwentySix}
                    expressor={
                        TextAndFig{
                            text: "我说我有点被吓到，leocth 问我为什么，我想说 ”我一想到英国的大学可能全部会拒绝我就不好了“，正在想如何遣词造句",
                            src:static_url!("uneasy.jpg"),
                            fig_renderer: {CaptionedFigRenderer{caption: "和 leocth 聊天截图"}}
                        }
                    }
                />
                <Entry<Ss> hour={Hour::Fourteen} minute={Minute::TwentyEight} expressor="忘了，大概是在想关于 leocth 在英语社群里夹杂中文的什么想法，还是在想怎么帮他翻译" />
                <Entry<Ss> hour={Hour::Fourteen} minute={Minute::ThirtyEight} expressor="想发送 \"3dsimplex\" 纠正自己之前说的 \"2dsimplex\"，尽管记不清到底哪个才是 tetrahedron" />
                <Entry<Ss> hour={Hour::Fourteen} minute={Minute::FiftyNine} expressor="“在 html! macro 里真的能用双斜线注释吗”" />
                <Entry<Ss> hour={Hour::Fifteen} minute={Minute::Thirty} expressor="把 Enhance 变量名改成更合适的 RenderExtra，不想再在这个网页上记录写代码流水账了，好无聊" />
                <Entry<Ss> hour={Hour::Fifteen} minute={Minute::ThirtyEight} expressor="trait 叫 ExpressEntry，那么 implementor 就叫 TextPost，FigPost 之类的..." />
                <Entry<Ss> hour={Hour::Eighteen} minute={Minute::TwentyEight} expressor="在想待会要是把工作谈妥下来就去买个 Airpods" />
                <Entry<Html> hour={Hour::Nineteen} minute={Minute::Twenty} expressor=
                    {{
                        html!{
                            <>
                                {"在脑补 “四月之声” 的正反方论证"}<br/><br/>
                                {"反 （大意来自之前看过的一篇微博）:"}<br/>
                                {"黑白滤镜，BBC，blahblah"}<br/><br/>
                                {"正:"}<br/>
                                {"可是这些录音都是真实的"}<br/><br/>
                                {"反:"}<br/>
                                {"真实的也不能选取，要全面"}<br/><br/>
                                {"..."}<br/><br/>

                                {"想到一些细枝末节要是辩下去没完没了，可能通俗意义上的争吵或者辩论是一种先画靶再射箭的行为，并不是要辩明白什么，只是想压得对方说不出话或者把对方打一顿。"}
                            </>
                        }
                    }}
                />
                <Entry<Ss> hour={Hour::Twenty} minute={Minute::Seven} expressor="手机切到微信看看沈雨荷有没有说什么" />
                <Entry<TextAndMultipleFig<Ss, NoCaptionFigRenderer>>
                    hour={Hour::Twenty}
                    minute={Minute::Sixteen}
                    expressor={
                        TextAndMultipleFig{
                            text: "看到知乎上一个问题的回答觉得难受，人是有级别一说的",
                            figs: vec![
                        (NoCaptionFigRenderer, static_url!("flight-a.png")),
                        (NoCaptionFigRenderer, static_url!("flight-b.png"))

                    ]
                        }
                    }
                />
                <Entry<Ss> hour={Hour::Twenty} minute={Minute::FortyTwo} expressor="听南航空难黑匣子录音" />
                <Entry<Ss> hour={Hour::TwentyOne} minute={Minute::Ten} expressor="YouTube 视频标题：“又一场俄式空难，机长让15岁的儿子驾驶，一分钟后全机75人无一生还”" />
                <Entry<Ss> hour={Hour::TwentyOne} minute={Minute::TwentyEight} expressor="看 Doctor Strange trailer 下面的评论区" />
                <Entry<Ss> hour={Hour::TwentyOne} minute={Minute::FortyFour} expressor="b站视频，28岁以来不刷牙，牙结石，恶心" />
                <Entry<Ss> hour={Hour::TwentyOne} minute={Minute::FortyNine} expressor="b站，看暴力摩托" />
                <Entry<TextAndFig<Ss, NoCaptionFigRenderer>>
                    hour={Hour::TwentyTwo}
                    minute={Minute::Eight}
                    expressor={
                        TextAndFig{
                            text: "b站广告太智障，这显然是宜家，荆门哪里有宜家？",
                            src:static_url!("jingmen-ikea.jpg"),
                            fig_renderer: {NoCaptionFigRenderer}
                        }
                    }
                />
                <Entry<Html> hour={Hour::TwentyTwo} minute={Minute::FortySeven} expressor=
                    {{
                        html!{
                            <>
                                {"有要做的事，比如去做面吃或者去改作业，但是在刷知乎，但是又没有认真看知乎，skim 完之后完全不理解说了什么。"}<br/><br/>
                                {"我还是喝点水之后去改作业吧。"}
                            </>
                        }
                    }}
                />

                {day_heading!(2022-5-7)}

                <Entry<Ss> hour={Hour::Seventeen} minute={Minute::One} expressor="leocth 说（steam 上的一个打折游戏）\"It's barely a dollar\"" />
                <Entry<Html> hour={Hour::Seventeen} minute={Minute::Eleven} expressor={{html!{<> {"在 YouTube 上看 Gay or European，"} <a href="https://youtu.be/XY9PmBNb3PE?t=174" target="_blank" rel="noopener">{"Look at condescending smirk..."}</a></>}}} />
                <Entry<Html> hour={Hour::Seventeen} minute={Minute::TwentySix} expressor={{html!{<><a href="https://www.youtube.com/watch?v=stXgn2iZAAY" target="_blank" rel="noopener">{"国会老爷审问 Mark Zuckerberg 之迷惑问题"}</a>{" 下面的一个热评：\"no wonder he looked mildly terrified the entire time he's confused as shit\" "}</>}}} />
                <Entry<Ss> hour={Hour::Eighteen} minute={Minute::FortyEight} expressor="我感觉泛型组件不值得" />
                <Entry<Ss> hour={Hour::Nineteen} minute={Minute::Two} expressor="在 beartracks 上看成绩单之类的出了没有" />
                <Entry<Html> hour={Hour::Nineteen} minute={Minute::TwentySix} expressor=
                    {{
                        html!{
                            <>
                                {"iory: \"I think it's a mental issue\" (指他老是紧张到无法呼吸)."}<br/>
                                {"me: \"ah great.\""}
                            </>
                        }
                    }}
                />
                <Entry<Html> hour={Hour::Twenty} minute={Minute::FiftyFour} expressor=
                    {{
                        html!{
                            <>
                                {"沈：“和完这个面我就得去洗澡了真的是要命”"}<br/>
                                {"我: *guttural 尬笑*"}
                            </>
                        }
                    }}
                />
                <Entry<Ss> hour={Hour::TwentyOne} minute={Minute::Three} expressor="和沈说大樱桃比小樱桃好" />
                <Entry<Ss> hour={Hour::TwentyOne} minute={Minute::Twenty} expressor="在吃金汤肥牛面，嚼里面的笋脆脆的，在想妈会不会觉得太辣" />
                <Entry<Ss> hour={Hour::TwentyOne} minute={Minute::ThirtyFive} expressor="小樱桃好酸" />
                <Entry<Ss> hour={Hour::TwentyOne} minute={Minute::FortyTwo} expressor="沈：“和面导致的精神崩溃然后自杀”" />
                <Entry<Ss> hour={Hour::TwentyTwo} minute={Minute::One} expressor="在摸鼻子" />
                <Entry<Ss> hour={Hour::TwentyTwo} minute={Minute::TwentySeven} expressor="虽然在看阿西莫夫的《基地》，但是脑子里在想黄色" />
                <Entry<Ss> hour={Hour::TwentyTwo} minute={Minute::ThirtyNine} expressor="尝试弄明白 Hollow Knight Hornet Theme 里中间的 polyrhythm" />
                <Entry<Ss> hour={Hour::TwentyThree} minute={Minute::Sixteen} expressor="看 Hollow Knight Hornet Theme 的谱子" />
                <Entry<Ss> hour={Hour::TwentyThree} minute={Minute::FiftyOne} expressor="左手摸到床上的《基地》打算拿起来继续读" />

                {day_heading!(2022-5-8)}

                <Entry<TextAndFig<Ss, NoCaptionFigRenderer>>
                    hour={Hour::Nine}
                    minute={Minute::Three}
                    expressor={
                        TextAndFig{
                            text: "\"crab 🦀\"",
                            src:static_url!("minecraft-crab.png"),
                            fig_renderer: {NoCaptionFigRenderer}
                        }
                    }
                />

                <Entry<TextAndFig<Ss, NoCaptionFigRenderer>>
                    hour={Hour::Nine}
                    minute={Minute::FiftyOne}
                    expressor={
                        TextAndFig{
                            text: "bleat 是什么意思？",
                            src:static_url!("fawn-bleats.jpg"),
                            fig_renderer: {NoCaptionFigRenderer}
                        }
                    }
                />
                <Entry<Ss> hour={Hour::Ten} minute={Minute::One} expressor="这集 two minute papers 竟然有12分钟" />
                <Entry<Ss> hour={Hour::Ten} minute={Minute::FiftyEight} expressor="在欣赏自己在知乎上写的 the HoTT Book 习题解答" />
                <Entry<Ss> hour={Hour::Eleven} minute={Minute::TwentyThree} expressor="看知乎回答，问题是“如何以“糟糕！修仙八百年，感染新冠病毒，我的身份曝光了！”为开头，写一个故事？”" />
                <Entry<Ss> hour={Hour::Eleven} minute={Minute::ThirtyFour} expressor="给解冻的鱼柳换水" />
                <Entry<Html> hour={Hour::Thirteen} minute={Minute::TwentyFive} expressor={{html!{<> {"磨咖啡, 在哼 "}<a href="https://youtu.be/7PYe57MwxPI?t=54" target="_blank" rel="noopener">{"I can't explain this kind of love..."}</a></>}}} />
                <Entry<Html> hour={Hour::Thirteen} minute={Minute::FortyThree} expressor={{html!{<> {"等待咖啡, 在唱 "}<a href="https://youtu.be/P1V6cQJpbc4?t=17" target="_blank" rel="noopener">{"and in July, a lemonade..."}</a></>}}} />
                <Entry<Ss> hour={Hour::Fourteen} minute={Minute::Nine} expressor="（在用 audacity）“100 hz 到 300 hz 大概是合适的音域范围”" />
                <Entry<Ss> hour={Hour::Fourteen} minute={Minute::TwentyThree} expressor="在听 I wish you love 的 钢琴 arrangement" />
                <Entry<Ss> hour={Hour::Fourteen} minute={Minute::FiftyThree} expressor="讶异于歌词里 \"I wish you\" 竟然是三连音" />
                <Entry<Ss> hour={Hour::Fifteen} minute={Minute::Two} expressor="就很开心" />
                <Entry<Html> hour={Hour::Fifteen} minute={Minute::FortySeven} expressor=
                    {{
                        html!{
                            <>
                                {"average judy enjoyer:"}<br/>
                                {"(smoke on the water)"}
                            </>
                        }
                    }}
                />
                <Entry<Ss> hour={Hour::Fifteen} minute={Minute::FiftyThree} expressor="抽象出来一个 container 装 Entry" />
                <Entry<Ss> hour={Hour::Sixteen} minute={Minute::TwentyFour} expressor="尝试拼写 d-i-f-f-i-c-u-l-t" />
                <Entry<Ss> hour={Hour::Sixteen} minute={Minute::Forty} expressor="数各个月是30天还是31天" />
                <Entry<Ss> hour={Hour::Sixteen} minute={Minute::FiftyEight} expressor="\"Was the pope a temporal mage?\"" />
                <Entry<Ss> hour={Hour::Seventeen} minute={Minute::ThirtySeven} expressor="厕所里" />
                <Entry<Ss> hour={Hour::Seventeen} minute={Minute::Fifty} expressor="做 YouTube 上的音感测试" />
                <Entry<Ss> hour={Hour::Eighteen} minute={Minute::Seventeen} expressor="电脑上输入法怎么也找不到”驶来“的”驶“" />
                <Entry<Ss> hour={Hour::Eighteen} minute={Minute::FortyFive} expressor="一些关于发际线的尴尬想法" />
                <Entry<Ss> hour={Hour::Nineteen} minute={Minute::Thirteen} expressor="忘了 (和亲戚在一起，抽不出空)" />
                <Entry<Ss> hour={Hour::Nineteen} minute={Minute::Nineteen} expressor="(和亲戚在一起，抽不出空)" />
                <Entry<Ss> hour={Hour::Twenty} minute={Minute::Eighteen} expressor="华强北的 Airpods 竟然能达到 8-9 小时的续航？？" />
                <Entry<Ss> hour={Hour::Twenty} minute={Minute::FortyThree} expressor="爹和司机说山竹好吃，我在吃榴莲" />
                <Entry<Ss> hour={Hour::TwentyOne} minute={Minute::ThirtySix} expressor="写 yew，琢磨 context 传递和 consume callback 啥的" />
                <Entry<Ss> hour={Hour::TwentyTwo} minute={Minute::TwentySeven} expressor="正在往支付宝里充值 1279 元人民币" />
                <Entry<Ss> hour={Hour::TwentyThree} minute={Minute::One} expressor="情不自禁跟着唱 “想把我唱给你听”" />


            </ContextProvider<Callback<Ss>>>
        </BounceRoot>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct YearHeadingProps {
    year: i32,
}


#[function_component(YearHeading)]
pub fn year_heading(props: &YearHeadingProps) -> Html {
    html! {
        <h1>{props.year.to_string()}</h1>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct DayHeadingProps {
    date: Date,
}

pub fn use_debounced_window_scrolling() -> bool {
    let state = use_state_eq(|| false);

    let debounce = {
        let state = state.clone();
        use_debounce(
            move || {
                state.set(false);
            },
            5,
        )
    };

    {
        let state = state.clone();
        use_event_with_window("scroll", move |_: Event| {
            state.set(true);
            debounce.run();
        });
    }


    *state
}

trait CssStyleDeclarationExt {
    fn margin_top(&self) -> String;
}

impl CssStyleDeclarationExt for CssStyleDeclaration {
    fn margin_top(&self) -> String {
        js_sys::Reflect::get(self, &JsValue::from_str("marginTop"))
            .unwrap()
            .unchecked_into::<JsString>()
            .into()
    }
}


#[function_component(DayHeading)]
pub fn day_heading(props: &DayHeadingProps) -> Html {
    let scrolling = use_debounced_window_scrolling();

    let sticky_date = use_slice_dispatch::<UpThereDates>();

    // #[cfg(debug_assertions)]
    // log::info!("debubbb");


    let h2 = use_node_ref();

    if scrolling{
        let up_there = {
            if let Some(h2) = h2.cast::<HtmlHeadingElement>() {
                let rect = h2.get_bounding_client_rect();

                let offset = 0f64;


                let document = gloo_utils::document_element();
                let window = gloo_utils::window();

                if let Ok(Some(style_dec)) = window.get_computed_style(&h2){
                    let margin_top_in_px = js_sys::parse_float(&style_dec.margin_top());
                    let inner_height: f64 = window.inner_height().unwrap().as_f64().unwrap();
                    let view_height = f64::max(document.client_height() as f64, inner_height);

                    !(rect.top() - margin_top_in_px).is_sign_positive()
                }else{
                    false
                }


            } else { false }
        };
        if up_there {
            sticky_date(StickDateUpdate::AddDateUpThere(props.date));
        } else {
            sticky_date(StickDateUpdate::RemoveDateDownBelow(props.date));
        }
    }


    html! {
        <h2 ref={h2}>{props.date.into_chinese_month_and_day()}</h2>
    }
}

pub trait IntoChineseMonthAndDay {
    fn into_chinese_month_and_day(self) -> String;
}

impl IntoChineseMonthAndDay for Date {
    fn into_chinese_month_and_day(self) -> String {
        let month: u8 = self.month().into();
        let chinese_month: String = month.to_lowercase_ten_thousand(ChineseVariant::Simple);
        let day: u8 = self.day();
        let chinese_day: String = day.to_lowercase_ten_thousand(ChineseVariant::Simple);

        format!("{}月{}日", chinese_month, chinese_day)
    }
}


pub trait ExpressEntry: PartialEq + Clone {
    fn expresses_entry(self) -> Html;
}

impl<T> ExpressEntry for T where T: Into<Html> + PartialEq + Clone {
    fn expresses_entry(self) -> Html {
        html! {
            <p>{self}</p>
        }
    }
}

// enum Figure{
//
//
// }

trait RenderFigure: PartialEq + Clone {
    fn render(&self, show_enlarged: Callback<Ss>, src: &'static str) -> Html {
        let class = use_style!(r"
            max-width: min(30rem, 100%);
            cursor: pointer;
        ");

        let image = html! {
            <img src={src} {class} loading="lazy" onclick= { move |_|                    show_enlarged.emit(src)}/>
        };

        self.render_context(image)
    }
    fn render_context(&self, image: Html) -> Html;
    // fn image_style(&self) -> Style{
    //     let class = use_style!(r"
    //         max-width: min(30rem, 100%);
    //         cursor: pointer;
    //     ");
    //     class
    // }
}


#[derive(PartialEq, Clone)]
struct CaptionedFigRenderer {
    caption: &'static str,
}

#[derive(PartialEq, Clone)]
struct NoCaptionFigRenderer;

impl RenderFigure for NoCaptionFigRenderer {
    fn render_context(&self, image: Html) -> Html {
        image
    }
}


impl RenderFigure for CaptionedFigRenderer {
    fn render_context(&self, image: Html) -> Html {
        let class = use_style!("margin: 0;");

        html! {
                <figure {class}>
                    {image}
                    <figcaption>{self.caption}</figcaption>
                </figure>
        }
    }
}


#[derive(PartialEq, Clone)]
struct TextAndFig<T, R> {
    text: T,
    fig_renderer: R,
    src: &'static str,
}


impl<T: PartialEq + Clone + Into<Html>, R: PartialEq + Clone + RenderFigure> ExpressEntry for TextAndFig<T, R> {
    fn expresses_entry(self) -> Html {
        let cb = use_context::<Callback<Ss>>();

        html! {
            if let Some(cb) = cb{
                <p>{self.text}</p>
                {self.fig_renderer.render(cb,self.src)}
            }
        }
    }
}

#[derive(PartialEq, Clone)]
struct TextAndMultipleFig<T, R> {
    text: T,
    figs: Vec<(R, &'static str)>,
}


impl<T: PartialEq + Clone + Into<Html>, R: PartialEq + Clone + RenderFigure> ExpressEntry for TextAndMultipleFig<T, R> {
    fn expresses_entry(self) -> Html {
        let cb = use_context::<Callback<Ss>>();

        html! {
            if let Some(cb) = cb{
                <p>{self.text}</p>
                {
                    for self.figs.clone().into_iter().map(|(fig, src)| {
                    html!{<>{fig.render(cb.clone(),src)}<br/></>}
                    })
                }

            }
        }
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct EntryProps<T: ExpressEntry> {
    hour: Hour,
    minute: Minute,
    expressor: T,
    // #[prop_or_default]
    // image_url: Option<&'static str>,
}

#[function_component(Entry)]
pub fn entry<T: ExpressEntry>(props: &EntryProps<T>) -> Html {
    html! {
        <>
            <h2>{format!("{:02}:{:02}", props.hour as u8, props.minute as u8)}</h2>
            {props.expressor.clone().expresses_entry()}
        </>
    }
}


fn main() {
    #[cfg(debug_assertions)]
    wasm_logger::init(wasm_logger::Config::default());
    set_event_bubbling(false);
    yew::start_app::<App>();
}
