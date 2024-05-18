use crate::components::navbar::Navbar;
use yew::prelude::*;
use web_sys::HtmlTextAreaElement;

#[derive(PartialEq, Properties)]
pub struct Props;

pub struct IndexPage {
    value: String,
    result: String,
}

pub enum Msg {
    InputChanged(String),
    ShowResult,
}

impl Component for IndexPage {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: String::from(""),
            result: String::from(""),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::InputChanged(new_value) => {
                self.value = new_value;
                true
            }
            Msg::ShowResult => {
                self.result = match bf_core::run(&self.value) {
                    Ok(s) => { s },
                    Err(e) => { format!("{:?}", e) }
                };
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let oninput = ctx.link().callback(|e: InputEvent| {
            let input: HtmlTextAreaElement = e.target_unchecked_into();
            Msg::InputChanged(input.value())
        });

        let onclick = ctx.link().callback(|_| Msg::ShowResult);

        html! {
            <>
                <Navbar />
                <div class="container mx-auto">
                    <div class="flex flex-col md:flex-row gap-6">
                        <div class="bg-white shadow-md rounded-lg p-4 flex-1">
                            <h2 class="text-2xl font-extrabold text-yellow-500 text-xl font-bold mb-4">{"Input"}</h2>
                            <textarea
                                id="textInput"
                                class="w-full p-2 border border-gray-300 rounded"
                                rows="6"
                                placeholder="Enter brainfuck code here..."
                                oninput={oninput}
                                value={self.value.to_string()}
                            />
                        </div>

                        <button
                        onclick={onclick}
                        class="group relative h-12 w-48 text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 font-medium rounded-lg text-sm px-4 py-2 mr-2 dark:bg-blue-600 dark:hover:bg-blue-700 focus:outline-none dark:focus:ring-blue-800 inline-block">
                            {"Run!!"}
                        </button>

                        <div class="bg-white shadow-md rounded-lg p-4 flex-1">
                            <h2 class="text-2xl font-extrabold text-yellow-500 text-xl font-bold mb-4">{"Result"}</h2>
                            <div id="parsedResult" class="w-full p-2 border border-gray-300 rounded h-48 bg-gray-50"></div>
                            {self.result.to_string()}
                        </div>
                    </div>
                </div>
            </>
        }
    }
}
