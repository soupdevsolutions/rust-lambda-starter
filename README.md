# rust-lambda-starter

This is a template for starting serverless projects with AWS Lambda, Rust, and Tofu.

## Components

The project contains:
- a Cargo project with an empty library and a **hello_world** binary representing an example Lambda function;
- a Python script for building all the Lambda funtions present in the `./src/bins` directory;
- a base infrastructure written in OpenTofu with IAM, Lambda, and API Gateway resources;
- a CI pipeline that triggers on any PR against the `main` branch or any commit to the `main` branch;
- a CD pipeline that triggers on any push to the `main` branch.

## Usage

### Getting the project

To get a local copy of the template, either use [pont](https://github.com/soupdevsolutions/pont):

```bash
pont build --name <your-project-name> --from https://github.com/soupdevsolutions/rust-lambda-starter
```

or clone the repository and change the occurences of the template name:

```bash
git clone https://github.com/soupdevsolutions/rust-lambda-starter ./<your-project-name>
cd <your-project-name>

# rename the occurences using bash or your IDE tooling
TEMPLATE_NAME_KEBAB_CASE=rust-lambda-starter
TEMPLATE_NAME_SNAKE_CASE=rust_lambda_starter
PROJECT_NAME=<your-project-name>

find . \( -name \*.rs -o -name \*.tf -o -name \*.toml -o -name \*.md -o -name \*.lock \) -exec sed -i '' "s#$TEMPLATE_NAME_KEBAB_CASE#$PROJECT_NAME#g" {} \;
find . \( -name \*.rs -o -name \*.tf -o -name \*.toml -o -name \*.md -o -name \*.lock \) -exec sed -i '' "s#$TEMPLATE_NAME_SNAKE_CASE#$PROJECT_NAME#g" {} \;
```

### Deployment

The recommended way for deploying the project is to use GitHub Actions; however, you can also do it from your local machine.  

For deploying with GitHub Actions, push your project to a new repository, go to the repository's **Settings** -> **Secrets and variables** -> **Actions**, and add secrets for your IAM user credentials:
- AWS_ACCESS_KEY_ID -> the access key id
- AWS_SECRET_ACCESS_KEY -> the secret access key
Any push to the `main` branch will trigger both the CI and CD pipelines.

If you choose to deploy from your local machine, make sure to follow the guide for [configuring the AWS CLI](https://docs.aws.amazon.com/cli/v1/userguide/cli-chap-configure.html), so OpenTofu can deploy to your AWS account. Then, you can run all the steps from the CD pipeline locally:

```bash
# init OpenTofu (only needs to be done once locally)
tofu -chdir=infrastructure init

# build the Lambda functions
python3 ./scripts/build.py

# plan and apply the changes
tofu -chdir=infrastructure plan
tofu -chdir=infrastructure apply
```
