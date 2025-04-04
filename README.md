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

For grpc to work you need to install protobuf on your system
You can get the latest version on github at https://github.com/protocolbuffers/protobuf/releases
Download a file that matches your requirements I chose protoc-30.2-linux-x86_64.zip but if you need to customize protobuf install protobuf-30.2.tar.gz or protobuf-30.2.zip
then unzip the file which can be done in the terminal with $ unzip protoc-30.2-linux-x86_64.zip -d protoc-30.2 $
After that, add it to the path by moving the protoc file from ~/Downloads/protoc-30.2/bin to the ~/bin and copy over the include folder usin $ cp -r protoc-30.2/include /usr/local/ $
Ensure that it works by runnin $ protoc --version $ on your terminal

If you dont know what is going on in that file then he has another video about grpcs in general

