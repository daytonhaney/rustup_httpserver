
use yew::prelude::*;

enum Msg {
    AddOne
}

struct CounterComponet{
    count: i64,
}
impl Componet for CounterComponet {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Contxt<Self>) -> Self{
        self { count: 0 }
    }
    fn update(&mut self,_ctx: &Contex<Self>, msg: Self::Message) {
        match msg{
            Msg::AddOne =>{
                self.count += 1;
                true 
            }
        }   
     }
     fn view(&self,ctx: &Context<Self>) ->Html{
        html!{
            <div class="container">
                <p>( self.count )</p>
                <button> ( "+1" ) </button>
            </div>    
        }
    }
}

