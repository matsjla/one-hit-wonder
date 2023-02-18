import * as apigateway from "@aws-cdk/aws-apigateway";
import * as certificatemanager from "@aws-cdk/aws-certificatemanager";
import * as lambda from "@aws-cdk/aws-lambda";
import * as route53 from "@aws-cdk/aws-route53";
import * as route53targets from "@aws-cdk/aws-route53-targets";
import * as cdk from "@aws-cdk/core";
import * as path from "node:path";

export class OneHitWonderStack extends cdk.Stack {
  constructor(scope: cdk.Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);
    const code = path.resolve(
      __dirname,
      "../../ohw-server/target/lambda/ohw-server"
    );
    const handler = new lambda.Function(this, "ohw-server-handler", {
      code: lambda.Code.fromAsset(code),
      functionName: "ohw-server-handler",
      handler: "main",
      runtime: lambda.Runtime.PROVIDED_AL2,
    });
    const zone = route53.HostedZone.fromLookup(this, "ohw-jun-codes-zone", {
      domainName: "jun.codes",
    });
    const certificate = new certificatemanager.DnsValidatedCertificate(
      this,
      "ohw-certificate",
      {
        domainName: `ohw-server.jun.codes`,
        hostedZone: zone,
        region: "us-east-1",
      }
    );
    const api = new apigateway.LambdaRestApi(this, "ohw-api", {
      handler: handler,
      domainName: {
        domainName: "ohw-server.jun.codes",
        certificate,
        securityPolicy: apigateway.SecurityPolicy.TLS_1_2,
        endpointType: apigateway.EndpointType.EDGE,
      },
      apiKeySourceType: apigateway.ApiKeySourceType.HEADER,
      deployOptions: {
        throttlingBurstLimit: 3,
        throttlingRateLimit: 3,
      },
    });
    const record = new route53.ARecord(this, "ohw-server-api-record", {
      recordName: "ohw-server",
      zone,
      target: route53.RecordTarget.fromAlias(
        new route53targets.ApiGateway(api)
      ),
    });
  }
}
