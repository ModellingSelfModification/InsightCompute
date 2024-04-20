use rand::prelude::*;

#[derive(Copy, Clone)]
enum  Modifier{
    Add
}

#[derive(Copy, Clone)]
enum Modified{
    Compute,
    Data
}

#[derive(Copy, Clone)]
struct Insight{
    modifier: Modifier,
    modified: Modified,
    value: f64,

}
#[derive( Clone)]
struct State {
    compute: f64,
    data: f64,
    insight_efficiency: f64,
    possible_insights: Vec<Insight>,   
}



impl State {

    fn find_insights(&mut self) -> Vec<Insight>{
        let mut new_insights = Vec::new();
        let mut old_insights = Vec::new();
        let mut rng = rand::thread_rng();

        for insight in self.possible_insights.iter(){
            /*Probability of discovering insight = data/compute * insight efficiency */
            let y: f64 = rng.gen();
            if y < (self.data/self.compute) * self.insight_efficiency {
                new_insights.push(insight.clone());
            } else {
                old_insights.push(insight.clone());
            }

        }
        self.possible_insights = old_insights;
        return new_insights;
    }

    fn apply_insight( &mut  self , insight : &Insight) -> () {
        match insight.modified{
            Modified::Compute => self.compute = match insight.modifier{
                                                Modifier::Add => self.compute + insight.value
                },
            Modified::Data => self.data = match insight.modifier{
                                                Modifier::Add => { self.add_insights(insight.value); self.data + insight.value}
            }
        }
    }

    fn add_insights(&mut self, data_added: f64) ->() {
      //  let insights_per_unit_data = 2;
        let mut amount_added = 0.0;
        while amount_added < data_added {
            self.possible_insights.push(Insight{modifier: Modifier::Add, modified: Modified::Compute,value: 1.0});
            self.possible_insights.push(Insight{modifier: Modifier::Add, modified: Modified::Data,value: 1.0});
            self.possible_insights.push(Insight{modifier: Modifier::Add, modified: Modified::Data,value: 1.0});
            amount_added +=1.0;
        }
    }

    fn step(&mut self) {
        let insights = self.find_insights();
        for insight in insights.iter() {
           self.apply_insight(insight);
        } 

    }
}

fn main() {
    println!("Hello, world!");
    let mut insights = Vec::new();
    insights.push(Insight{modifier: Modifier::Add, modified: Modified::Compute,value: 1.0});
    insights.push(Insight{modifier: Modifier::Add, modified: Modified::Data,value: 1.0});
    let mut state = State {compute: 1.0, data: 1.0, insight_efficiency: 1.0,  possible_insights: insights};
    for x in 1..8 {
        state.step();
                 
        println!("timestamp {}", x);
        println!("{} compute", state.clone().compute);
        println!("{} data", state.clone().data);
        println!("{} insights", state.clone().possible_insights.len());

    }
}
