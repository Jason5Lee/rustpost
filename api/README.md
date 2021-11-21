The REST API service of the rustpost.

## Design

### File Sturecture

* It uses [the Vertical Slice Architecture](https://jimmybogard.com/vertical-slice-architecture/). Each workflow, which corresponds to a procedure, has its code in a folder with the name of workflow. In particular, the `common` folder has the code that may be used in each workflow.
* In each folder,
  * the `mod.rs` file belongs to business logic domain. It includes the definitions of the models. Thanks to the expressiveness of the Rust type-system, for each model, the code can clearly show its fields, whether it can be optional, what cases it has, and what constraint it has. For the constrained value, it is represented by a wrapper with a private field. The type can only be constructed after validation. If you're communicating with product managers or domain experts, or if you're writing user manual, you can work with this file.
  * the `api.rs` file includes the information of the API. It includes the information of HTTP method, path, resquest definition, response status and response definition. It also includes how a request is converted into the workflow input and how the workflow output converts to the response which explains the meaning of the request and response fields. If you're working with the API document or if you're the client who calls the API, you can work with this file.
  * the `error.rs` file includes the error message and the status code of the user error which will be explained later. If you're working with the API document or if you're the client who calls the API, you can work with this file for error documentation or handling.
  * the `deps.rs` file includes the implementation of the dependent steps. It usually includes the details of persistence.
  * the `tests.rs` file includes the unit-tests. As this is just a sample project instead of a production one, only `delete_post` contains unit-test.

### Error Design

If the source of an error is the user's input, it's an user error. 

For example, if the request misses a required field, it is NOT an user error, but a issue of the client implementation. The client might put it in the wrong field or not let user to input at all. This error cannot be fixed if user provides a different input. If the request has a field, but the value is invalid or the resource cannot be found, then it is an user error. The client has properly constructs the request, but the value of a field which is from the user input is invalid. This error can be fixed by user providing a different input.

The user error often requires some presentations at client. The client should present what and how an input is invalid to user, so user can fix it properly. For other errors, since the user has no way to fix it, they may just be flagged as a bug or produce an alert. Because the user errors need to be handled, their definitions is put in the `error.rs` , so that the client can see the message and know how to identify them.

In particular for the invalid of the request, if the error is an user error, it responds `422 Unprocessable Entity`, otherwise if it indicates the wrong implementation of client, it responds `400 Bad Request`.

### Dependent Steps and mocking

It defines a macro for defining a `Steps` struct. With `#[cfg(test)]`, the struct `Deref` to an async trait object with the methods of the dependent steps, which allows you to use mock implementation. With `#[cfg(not(test))]`, it has the methods of the dependent steps which delegates to the implementations in `deps.rs`. You can see `delete_post` as an example.

## Deployment

The project read the config from environment variables with `.env` supports. An example of the environment can be found at [here](./.env).

This projects use MySQL as the persistence, [`setup.sql`](./setup.sql) for setting up and [`undo.sql`](./undo.sql) for undo the setting.
