use crate::components::{axis::Direction, Axis};
use crate::lib::profile::AnalyzedProfile;
use crate::lib::scale;
use crate::prelude::*;

static OUTER: (f64, f64) = (1024., 480.);
static INNER: (f64, f64, f64, f64) = (30., 20., 1004., 450.);

#[derive(Properties, PartialEq)]
pub struct Props {
    pub data: AnalyzedProfile,
}

#[derive(PartialEq)]
pub struct Graph;

impl Component for Graph {
    type Message = ();
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <svg width={ format!("{}px", OUTER.0) } height={ format!("{}px", OUTER.1) } viewBox={ format!("0 0 {} {}", OUTER.0, OUTER.1) }>
                {self.view_axis(ctx)}
                <g>
                    {self.view_graph_temperature(ctx)}
                    {self.view_graph_pressure(ctx)}
                    {self.view_graph_flow(ctx)}
                </g>
            </svg>
        }
    }
}

impl Graph {
    fn view_axis(&self, ctx: &Context<Self>) -> Html {
        html! {
            <g>
                <g transform={ format!("translate({},{})", INNER.0, INNER.3) }>
                    <Axis domain={(0., ctx.props().data.elapsed_time)} range={(0., INNER.2 - INNER.0)} dir={Direction::Horizontal} min_unit={10.0} />
                </g>
                <g transform={ format!("translate({},{})", INNER.0, INNER.3) }>
                    <Axis domain={(0., 12.)} range={(0., INNER.1 - INNER.3)} dir={Direction::Vertical} min_unit={1.0} />
                </g>
            </g>
        }
    }

    fn view_graph_temperature(&self, ctx: &Context<Self>) -> Html {
        let x = scale((0., ctx.props().data.elapsed_time), (INNER.0, INNER.2));
        let y = scale((20., 100.), (INNER.3, INNER.1));
        html! {
            ctx.props().data.temperature.iter().map(|(x1, y1, x2, y2)| {
                html! {
                    <line
                        x1={x(*x1).to_string()}
                        y1={y(*y1).to_string()}
                        x2={x(*x2).to_string()}
                        y2={y(*y2).to_string()}
                        stroke="darkred"
                        stroke-width="1.5px"
                        stroke-linecap="round"
                    />
                }
            }).collect::<Html>()
        }
    }

    fn view_graph_pressure(&self, ctx: &Context<Self>) -> Html {
        let x = scale((0., ctx.props().data.elapsed_time), (INNER.0, INNER.2));
        let y = scale((0., 12.), (INNER.3, INNER.1));
        html! {
            ctx.props().data.pressure.iter().map(|(x1, y1, x2, y2)| {
                html! {
                    <line
                        x1={x(*x1).to_string()}
                        y1={y(*y1).to_string()}
                        x2={x(*x2).to_string()}
                        y2={y(*y2).to_string()}
                        stroke="darkgreen"
                        stroke-width="1.5px"
                        stroke-linecap="round"
                    />
                }
            }).collect::<Html>()
        }
    }

    fn view_graph_flow(&self, ctx: &Context<Self>) -> Html {
        let x = scale((0., ctx.props().data.elapsed_time), (INNER.0, INNER.2));
        let y = scale((0., 12.), (INNER.3, INNER.1));
        html! {
            ctx.props().data.flow.iter().map(|(x1, y1, x2, y2)| {
                html! {
                    <line
                        x1={x(*x1).to_string()}
                        y1={y(*y1).to_string()}
                        x2={x(*x2).to_string()}
                        y2={y(*y2).to_string()}
                        stroke="darkblue"
                        stroke-width="1.5px"
                        stroke-linecap="round"
                    />
                }
            }).collect::<Html>()
        }
    }
}
