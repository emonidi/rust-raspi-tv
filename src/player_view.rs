use orbtk::prelude::*;

#[derive(Debug, Copy, Clone)]
enum Action {
    
}

#[derive(Debug, AsAny, Default)]
pub struct PlayerViewState {
    // channel:Channel
}


impl PlayerViewState {
    // add code here
}


impl State for PlayerViewState{

}

widget!(PlayerView<PlayerViewState>);

impl Template for PlayerView{
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("PlayerView")
    }
}