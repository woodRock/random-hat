<<<<<<< HEAD
use crate::routes::Route;
=======
>>>>>>> 6345a0cba85677e4b142caa2251896a883202e9f
/// Home - home.rs
/// ==============
/// This is the home page. It is the first page that the user sees when they visit the website.
use yew::prelude::*;
<<<<<<< HEAD
use yew_router::prelude::*;
// use log::info;

use yew::{html, Callback};

#[function_component(Home)]
pub fn home() -> Html {
    let history = use_history().unwrap();
    let see_students = Callback::once(move |_| history.push(Route::Students));

    const DEFAULT: &str = "???";
    const SUPER_RANDOM_STUDENT: &str = "Jordan MacLachlan: Evolutionary Computation for Emergency Medical Services Routing";

    let selected_student = use_state(|| String::from(DEFAULT));
    let onclick = {
        let selected_student = selected_student.clone();
        Callback::from(move |_| selected_student.set(String::from(SUPER_RANDOM_STUDENT)))
    };
    let reset = {
        let selected_student = selected_student.clone();
        Callback::from(move |_| selected_student.set(String::from(DEFAULT)))
    };

    html! {
        <div>
            <h1>{ "Random Hat 🎩" } </h1>
            <p> { "A random name generator for selecting FASLIP nominees each week."} </p>
            <h3> { "Click the big hat icon to (psuedo) randomly select a speaker for next week!" } </h3>
            <button onclick={onclick}> <h1> { "🎩" } </h1> </button>
            <h1> {selected_student.to_string()}</h1>
            <button onclick={reset}>{"Reset"}</button>
            <p></p>
            <button onclick={see_students}>{"See FASLIP students"}</button>
=======
use yew::{html, Callback};
// DEBUG ONLY!!!
use log::info;

#[function_component(Home)]
pub fn home() -> Html {

    // Stolen x: https://stackoverflow.com/questions/75350345/show-input-value-in-browser-using-yew-rust
    let name = use_state(|| String::new());

    html! {
        <div>
            <h1>{ "How doomer are you?" } </h1>
            <p> { "For each term, respond to how much your beliefs align with the term, and the definition provided here."} </p>
            <p> { "Note: only works on a desktop computer (for now!) 🖥️"} </p>

            <h3>{ "Transhumanism" }</h3>
            <p> { "The belief that humans can and should use technology to improve themselves (CoPilot)." } </p>
            <div id="position-buttons-for-user">
                <button onclick={Callback::from(|_| (
                        info!("Transhumanism - Strongly disagree (-2)")
                    ))}>
                    //      ^^^^^^^ event listener name
                    { "Strongly disagree" }
                </button>
                <button onclick={Callback::from(|_| (
                    info!("Transhumanism - Disagree  (-1)")
                ))}>
                //      ^^^^^^^ event listener name
                {  "Disagree" }
                </button>
                <button onclick={Callback::from(|_| (
                    info!("Transhumanism - Neutral (0)")
                ))}>
                //      ^^^^^^^ event listener name
                {  "Neutral" }
                </button>
                <button onclick={Callback::from(|_| (
                    info!("Transhumanism - Agree (+1)")
                ))}>
                //      ^^^^^^^ event listener name
                {  "Agree" }
                </button>
                <button onclick={Callback::from(|_| (
                    info!("Transhumanism - Strongly Agree (+2)")
                ))}>
                //      ^^^^^^^ event listener name
                {  "Strongly Agree" }
                </button>
            </div>

            <h3>{ "Extropianism" }</h3>
            <p> { "Evolving framework of values and standards for continuously improving the human condition (Wikipedia)." } </p>
            <div id="position-buttons-for-user">
                <button onclick={Callback::from(|_| (
                        info!("Extropianism - Strongly disagree (-2)")
                    ))}>
                    //      ^^^^^^^ event listener name
                    { "Strongly disagree" }
                </button>
                <button onclick={Callback::from(|_| (
                    info!("Extropianism - Disagree  (-1)")
                ))}>
                //      ^^^^^^^ event listener name
                {  "Disagree" }
                </button>
                <button onclick={Callback::from(|_| (
                    info!("Extropianism - Neutral (0)")
                ))}>
                //      ^^^^^^^ event listener name
                {  "Neutral" }
                </button>
                <button onclick={Callback::from(|_| (
                    info!("Extropianism - Agree (+1)")
                ))}>
                //      ^^^^^^^ event listener name
                {  "Agree" }
                </button>
                <button onclick={Callback::from(|_| (
                    info!("Extropianism - Strongly Agree (+2)")
                ))}>
                //      ^^^^^^^ event listener name
                {  "Strongly Agree" }
                </button>
            </div>

            <h3>{ "Singularitarianism" }</h3>
            <p> { "The belief that the development of artificial intelligence will lead to a technological singularity (CoPilot)." } </p>
            <div id="position-buttons-for-user">
                <button onclick={Callback::from(|_| (
                        info!("Singularitarianism - Strongly disagree (-2)")
                    ))}>
                    //      ^^^^^^^ event listener name
                    { "Strongly disagree" }
                </button>
                <button onclick={Callback::from(|_| (
                    info!("Singularitarianism - Disagree  (-1)")
                ))}>
                //      ^^^^^^^ event listener name
                {  "Disagree" }
                </button>
                <button onclick={Callback::from(|_| (
                    info!("Singularitarianism - Neutral (0)")
                ))}>
                //      ^^^^^^^ event listener name
                {  "Neutral" }
                </button>
                <button onclick={Callback::from(|_| (
                    info!("Singularitarianism - Agree (+1)")
                ))}>
                //      ^^^^^^^ event listener name
                {  "Agree" }
                </button>
                <button onclick={Callback::from(|_| (
                    info!("Singularitarianism - Strongly Agree (+2)")
                ))}>
                //      ^^^^^^^ event listener name
                {  "Strongly Agree" }
                </button>
            </div>
        
            <h3>{ "Cosmism" }</h3>
            <p> { "The doctrine that the material universe works automatically; affirmative atheism (Century Dictionary)." } </p>
            <div id="position-buttons-for-user">
                <button onclick={Callback::from(|_| (
                    info!("Cosmism - Strongly disagree")
                ))}>
                    //      ^^^^^^^ event listener name
                    { "Strongly disagree" }
                </button>
                <button onclick={Callback::from(|_| (
                    info!("Cosmism - Disagree  (-1)")
                ))}>
                //      ^^^^^^^ event listener name
                {  "Disagree" }
                </button>
                <button onclick={Callback::from(|_| (
                    info!("Cosmism - Neutral (0)")
                ))}>
                //      ^^^^^^^ event listener name
                {  "Neutral" }
                </button>
                <button onclick={Callback::from(|_| (
                    info!("Cosmism - Agree (+1)")
                ))}>
                //      ^^^^^^^ event listener name
                {  "Agree" }
                </button>
                <button onclick={Callback::from(|_| (
                    info!("Cosmism - Strongly Agree (+2)")
                ))}>
                //      ^^^^^^^ event listener name
                {  "Strongly Agree" }
                </button>
            </div>

            <h3>{ "Rationalism" }</h3>
            <p> { "The belief that reason is the chief source and test of knowledge (CoPilot)." } </p>
            <div id="position-buttons-for-user">
                <button onclick={Callback::from(|_| (
                    info!("Rationalism - Strongly disagree (-2)")
                ))}>
                    //      ^^^^^^^ event listener name
                    { "Strongly disagree" }
                </button>
                <button onclick={Callback::from(|_| (
                    info!("Rationalism - Disagree  (-1)")
                ))}>
                //      ^^^^^^^ event listener name
                {  "Disagree" }
                </button>
                <button onclick={Callback::from(|_| (
                    info!("Rationalism - Neutral (0)")
                ))}>
                //      ^^^^^^^ event listener name
                {  "Neutral" }
                </button>
                <button onclick={Callback::from(|_| (
                    info!("Rationalism - Agree (+1)")
                ))}>
                //      ^^^^^^^ event listener name
                {  "Agree" }
                </button>
                <button onclick={Callback::from(|_| (
                    info!("Rationalism - Strongly Agree (+2)")
                ))}>
                //      ^^^^^^^ event listener name
                {  "Strongly Agree" }
                </button>
            </div>

            <h3>{ "Effective Altruism (EA)" }</h3>
            <p>{ "Sam Bankman-Fried appears to have engaged in extreme misconduct precisely because he believed in utilitarianism and effective altruism, and that his mostly EA-affiliated colleagues at FTX and Alameda Research went along with the plan for the same reasons (Vox)."} </p>
            <div id="position-buttons-for-user">
                <button onclick={Callback::from(|_| (
                    info!("EA - Strongly disagree (-2)")
                ))}>
                    //      ^^^^^^^ event listener name
                    { "Strongly disagree" }
                </button>
                <button onclick={Callback::from(|_| (
                    info!("EA - Disagree  (-1)")
                ))}>
                //      ^^^^^^^ event listener name
                {  "Disagree" }
                </button>
                <button onclick={Callback::from(|_| (
                    info!("EA - Neutral (0)")
                ))}>
                //      ^^^^^^^ event listener name
                {  "Neutral" }
                </button>
                <button onclick={Callback::from(|_| (
                    info!("EA - Agree (+1)")
                ))}>
                //      ^^^^^^^ event listener name
                {  "Agree" }
                </button>
                <button onclick={Callback::from(|_| (
                    info!("EA - Strongly Agree (+2)")
                ))}>
                //      ^^^^^^^ event listener name
                {  "Strongly Agree" }
                </button>
            </div>

            <h3>{ "Longtermism" }</h3>
            <p> { "The belief that the long-term future of humanity is more important than the short-term (CoPilot)." } </p>
            <div id="position-buttons-for-user">
                <button onclick={Callback::from(|_| (
                    info!("Longtermism - Strongly disagree (-2)")
                ))}>
                    //      ^^^^^^^ event listener name
                    { "Strongly disagree" }
                </button>
                <button onclick={Callback::from(|_| (
                    info!("Longtermism - Disagree  (-1)")
                ))}>
                //      ^^^^^^^ event listener name
                {  "Disagree" }
                </button>
                <button onclick={Callback::from(|_| (
                    info!("Longtermism - Neutral (0)")
                ))}>
                //      ^^^^^^^ event listener name
                {  "Neutral" }
                </button>
                <button onclick={Callback::from(|_| (
                    info!("Longtermism - Agree (+1)")
                ))}>
                //      ^^^^^^^ event listener name
                {  "Agree" }
                </button>
                <button onclick={Callback::from(|_| (
                    info!("Longtermism - Strongly Agree (+2)")
                ))}>
                //      ^^^^^^^ event listener name
                {  "Strongly Agree" }
                </button>
            </div>
            <h1> { "Finished?" }</h1>
            <p> { "If you have finished, press F12 to open the console, tally the points from your answers, and you should have a number!" } </p>
            <h2> { "Results" } </h2>
            <div id="results">
                <table>
                <thead>
                    <tr>
                    <th> { "Category" }</th>
                    <th>{"Score Range"}</th> 
                    <th>{"Person"}</th> 
                    <th>{"Comment"}</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>{"Bloomer"}</td>
                        <td>{"-14 or lower"}</td>
                        <td>{"Sam Altman"}</td>
                        <td>{"Prepare to be ALIGNED!"}</td>
                    </tr>
                    <tr>
                        <td>{"Bloom-curious"}</td>
                        <td>{"-13 to -8"}</td>
                        <td>{"Yann LeCun"}</td>
                        <td>{"'AI doomism is quickly becoming indistinguishable from an apocalyptic religion.' ~ Yann LeCun"}</td>
                    </tr>
                    <tr>
                        <td>{"NPC"}</td>
                        <td>{"-7 to 7"}</td>
                        <td>{"Elon Musk"}</td>
                        <td>{"Stick to cars, and rockets, and internet, future king of Mars! 👑"}</td>
                    </tr>
                    <tr>
                        <td>{"Doom-curious"}</td>
                        <td>{"8 to 13"}</td>
                        <td>{"Nick Bostrom"}</td>
                        <td>{"Terminator was good... but it was just a movie! 😎"}</td>
                    </tr>
                    <tr>
                        <td>{"Doomer"}</td>
                        <td>{"13+"}</td>
                        <td>{ "Eliezer Yudkowsky" }</td>
                        <td>{"Please unblock me on Twitter! ❤️"}</td>
                    </tr>
                </tbody>
                </table>
            </div>
>>>>>>> 6345a0cba85677e4b142caa2251896a883202e9f
        </div>
    }
}
