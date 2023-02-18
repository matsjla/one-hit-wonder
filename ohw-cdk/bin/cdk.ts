#!/usr/bin/env node
import * as cdk from "@aws-cdk/core";
import { OneHitWonderStack } from "../lib";

const app = new cdk.App();
new OneHitWonderStack(app, "OneHitWonderStack", {
  env: {
    account: process.env.CDK_DEFAULT_ACCOUNT,
    region: "eu-west-2",
  },
  tags: {
    project: "ohw",
  },
});
