# GrpcWithTonic
Makeing a Grpc with login and authentication in rust using tonic

## Dev Diary

### Day 1 
I start by following the instructions in https://www.youtube.com/watch?v=kerKXChDmsE to get a feel for what can be done.
Tonic has automated protobuf compilation. Protobuff stands for protocoll buffer and it defines the structure of your data like JSON but binary as well as the API.
Tonic also uses macros for importing the messages sent with protobuff.

I creaate the calculator_tutorial project with cargo
THen I add the tonic, tokio and prost crate
Then I add tonic build as a build dependency with $ cargo add tonic-build --build $
Then I create a protobuf definition by creating a proto folder and a protofile like calculator_tutorial/proto/calculator.proto
For language support I installed proto-3 as a vscode addon

If you dont know what is going on in that file then he has another video about grpcs in general

