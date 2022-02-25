use crate::lib::scale;
use crate::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub domain: (f64, f64),
    pub range: (f64, f64),
    pub dir: Direction,
    pub min_unit: f64,
}

#[derive(PartialEq)]
pub struct Axis;

#[derive(Debug, PartialEq)]
pub enum Direction {
    Horizontal,
    Vertical,
}

impl Component for Axis {
    type Message = ();
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match ctx.props().dir {
            Direction::Horizontal => {
                html! {
                    <g>
                        <line
                            x1="0."
                            y1="0."
                            x2={ctx.props().range.1.to_string()}
                            y2="0."
                            stroke="darkgray"
                            stroke-width="1.25px"
                            stroke-linecap="round"
                        />
                        { self.view_scale(ctx) }
                    </g>
                }
            }
            Direction::Vertical => {
                html! {
                    <g>
                        <line
                            x1="0."
                            y1="0."
                            x2="0."
                            y2={ctx.props().range.1.to_string()}
                            stroke="darkgray"
                            stroke-width="1.25px"
                            stroke-linecap="round"
                        />
                    </g>
                }
            }
        }
    }
}

impl Axis {
    fn view_scale(&self, ctx: &Context<Self>) -> Html {
        match ctx.props().dir {
            Direction::Horizontal => {
                let x = scale(ctx.props().domain, ctx.props().range);
                let mut t = ctx.props().min_unit;
                let mut items = vec![];
                loop {
                    items.push(html! {
                        <line
                            x1={x(t).to_string()}
                            y1="0."
                            x2={x(t).to_string()}
                            y2="10."
                            stroke="darkgray"
                            stroke-width=".75px"
                            stroke-linecap="round"
                        />
                    });

                    t += ctx.props().min_unit;
                    if t > ctx.props().domain.1 {
                        break;
                    }
                }
                html! { for items }
            }
            _ => html! {},
        }
    }
}
