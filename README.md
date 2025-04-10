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

### Database Access

* create docker compose file in login_server/db/docker-compose.yml
* create connection string env variables in login_server/.cargo/config.toml
* make functions to get env variables in login_server/src/get_env.rs
* add tokio -F full and sqlx -F "postgres", "runtime-tokio-rustls", "uuid" and "time"
* create sql files in login_server/db/sql
* create main error and publically use it in main
* create data_access module with db_setup with mod and tests
* add the crate serial_test as a dev dependency
* create the tests for db setup in login_server/src/data_access/db_setup/tests.rs
* create a data access manager in login_server/src/data_access/mod.rs to hold the server state
* use the data access manager to create the server_server function in main 

### Setting up tracing

* add tracing and tracing-subscriber -F env-filter crates
* create module log and tracing inside and create enable tracing function
* run function enable_tracing in main
* also set RUST_LOG in login_server/.cargo/config.toml

### Standard Create Read and Delete functions

#### Proc-macro utils
* create utils module in login_server/src/utils/mod.rs
* create a proc-macro library in utils folder with $ cargo new proc-macros --lib $
* specify the proc-macros crate as a dependency in login_server/Cargo.toml
* cd into the proc-macros folder and add quote and syn crates
* create derive macros .../proc-macros/src/lib.rs for getting struct fields and to_hashmap
* create coresponding traits as the derive macro in login_server/src/utils/traits.rs

#### Basic user controllers
* add crate uuid -F v4 as the id's in the database are of that type.
* create the standard crd functions in login_server/src/data_access/base_crud.rs
* use the standard in a controller f.ex as in login_server/src/data_access/user_controller/mod.rs
* also create views for your table as in login_server/src/views/user.rs
* group the views through traits so the basic functions can be used to retrive different types
* creates tests as in login_server/src/data_access/user_controller/tests.rs

### Make test scripts

* install cargo make with $ cargo install cargo-make $
* create a make file like in login_server/Makefile.toml
// Im thinking of maybe creating a function that starts and stops the db
//  but It will probably be a timing hassle

### Encryption functions

#### Password hash
* create crypt module like login_server/src/crypt/mod.rs
* add argon2 crate
* create function to hash password like in login_server/src/crypt/mod.rs
* create public functions to validate and encrypt passwords like login_server/src/crypt/password.rs
* test password encryption and validation like in login_server/src/crypt/tests.rs

#### Jwt logic
* add blake2, time and base64 crates
* create base64 utils like in login_server/src/utils/base64.rs
* create encrypt with black2b function like in login_server/src/crypt/mod.rs
* create jwt struct and implement from str and display like in login_server/src/crypt/jwt.rs
* create time utils like in login_server/src/utils/time.rs
* make create jwt token, signature and validate functions like login_server/src/crypt/jwt.rs
* add a token key and durration to env like in login_server/.cargo/config.toml
* get the env variables and extract them in appropriate format like in login_server/src/get_env.rs
* implement new and validate jwt token functions like in login_server/src/crypt/jwt.rs
* test the jwt functions like in login_server/src/data_access/db_setup/tests.rs

### Login and Register functions

* create user for auth struct like in login_server/src/views/user.rs
* create register function like in login_server/src/data_access/user_controller/mod.rs
* create user for login and validation structs in login_server/src/views/user.rs
* create login function like in login_server/src/data_access/user_controller/mod.rs
* test like in login_server/src/data_access/user_controller/tests.rs

### gRPC

* add tonic and prost with cargo as I already have protobufs installed
* add tonic build as a build dependency with $ cargo add tonic-build --build $
* create the auth protobuf definition like login_server/proto/login_server.proto
// something cool would be a proc-macro that generated an automatic probuf definition form structs
* create a buld file like login_server/build.rs
* include the protobuf like in login_server/src/main.rs
* implement from error for the tonic status as in login_server/src/error.rs
* create an authentication service and implement it like in login_server/src/grpc/services/user.rs
* serve the service like in login_server/src/main.rs in serve_server

### Add middlewares to grpc

