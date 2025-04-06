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

Then create a build file like this in calculator_tutorial/build.rs

After that I included the calculatro protobuf in the proto module crated in src/main.rs and implemented the calculator service and served it in main.

### Day2 

Next we need reflection which is the ability for a service to communicate its grpc contracts to clients which eliminates the need of a client to have the protobuf definition. To do that we need the tonic-reflection crate. Then in /build.rs some code was added to make the out dir generate a file descriptor set. Then in the proto mod in main we include the file descriptor set as a const.
Then in src/main.rs a service is created which relays the description set to clients.

I then define the bins in cargo.toml, one for server which is src/main and one for client which is in src/client. This will specify two binaries so when you run $ cargo run --bin server $ it will run the server and when you run $ cargo run --bin client $ it will cun the code in /src/client.rs which generates a grpc request.

Next to demonstrate error hadling we add a devide method to the grpc. Incalculator_tutorial/proto/calculator.proto we add a divide rpc to the calculator service.
Then we in /src/main.rs we add a divide function to the implementation of the calculator service.
Crucially we return an error containing a tonic status to handle a division zero error to pervent the function from panicing. To demonstrate we also added a division reqest in calculator_tutorial/src/client.rs

To demonstrate tonics handling of state we create an arc read write type called state in main and then add a parameter called uses to the Calculator service struct. We the create an increment function and use it in both request implementations. Then we create a new service in calculator_tutorial/proto/calculator.proto whereafter we implement it in src/main.rs and share the state in the main function. Dont forget to add async trait on the implementation of the service trait. The I tested the code in client.rs. 

Then to demonstrate middleware aka interceptors we create a function whoch takes a tonic request and returns one too and checks the metadata or something in that request. Then instead of server::new in the server builder we add server::with_interceptor(service, interceptor) to have that interceptor run before the handler. A con however is that interceptors cannot be async. We also add the metadata in src/client.rs.

To run grpc on the web without a proxy server or such we add the tonic-wenb and tower-http -F cors crates. Add accept_http1 and a cors layer from tower to enable cors and it should be accessable from a frontend.

If you dont know what is going on in that file then he has another video about grpcs in general

## Login Server

### Database

//* add tokio -F full
* create docker compose file in login_server/db/docker-compose.yml
* create connection string env variables in login_server/.cargo/config.toml
* make functions to get env variables in login_server/src/get_env.rs
* add tokio -F full and sqlx -F "postgres", "runtime-tokio-rustls", "uuid" and "time"
* create sql files in login_server/db/sql
* create main error and publically use it in main
* create data_access module with db_setup with mod and tests