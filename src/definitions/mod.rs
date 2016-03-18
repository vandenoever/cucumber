use request::InvokeArgument;
use response::InvokeResponse;
use matcher::Matcher;

pub mod registration;

pub trait SendableStep<World>: Send + Fn(&Matcher<World>, &mut World, Vec<InvokeArgument>) -> InvokeResponse {}

impl<T, World> SendableStep<World> for T where T: Send + Fn(&Matcher<World>, &mut World, Vec<InvokeArgument>) -> InvokeResponse {}

pub type Step<World> = Box<SendableStep<World, Output=InvokeResponse>>;

pub type StepId = u32;
