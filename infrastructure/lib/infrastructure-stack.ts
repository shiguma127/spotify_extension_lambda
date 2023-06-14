import { aws_codepipeline_actions, Duration, Stack, StackProps } from 'aws-cdk-lib';
import { Cache, ComputeType, LinuxBuildImage, PipelineProject } from 'aws-cdk-lib/aws-codebuild';
import { Artifact, Pipeline } from 'aws-cdk-lib/aws-codepipeline';
import * as apigatewayv2 from '@aws-cdk/aws-apigatewayv2-alpha';
import { HttpLambdaIntegration } from '@aws-cdk/aws-apigatewayv2-integrations-alpha';
import * as lambda from 'aws-cdk-lib/aws-lambda';
import * as iam from 'aws-cdk-lib/aws-iam';
import * as s3 from "aws-cdk-lib/aws-s3";
import * as sns from 'aws-cdk-lib/aws-sns';
import * as subs from 'aws-cdk-lib/aws-sns-subscriptions';
import * as sqs from 'aws-cdk-lib/aws-sqs';
import * as dynamodb from 'aws-cdk-lib/aws-dynamodb'
import { Construct } from 'constructs';
import { Bucket } from 'aws-cdk-lib/aws-s3';
import { LogGroup } from 'aws-cdk-lib/aws-logs';
import { type } from 'os';



export class InfrastructureStack extends Stack {
  constructor(scope: Construct, id: string, props?: StackProps) {
    super(scope, id, props);

    const codeBucket = new s3.Bucket(this, "spotify-extension-src", {
      bucketName: "spotify-extension-src"
    })
    const lambdaFunction = new lambda.Function(this, "lambda-function", { runtime: lambda.Runtime.PROVIDED_AL2, handler: "bootstrap",
    code: lambda.Code.fromBucket(codeBucket, "bootstrap.zip") })

    // const lambdaLogGroup = new LogGroup(this, "asd", { logGroupName: `/aws/lambda/${lambdaFunction.functionName}` })

    const lambdaIntegration = new HttpLambdaIntegration("lambda-integration", lambdaFunction)
    const httpApi = new apigatewayv2.HttpApi(this, "ApiGateway", {})

    httpApi.addRoutes({
      path: '/{proxy+} ',
      methods: [ apigatewayv2.HttpMethod.ANY],
      integration: lambdaIntegration,
    });

    new dynamodb.Table(this, 'session-table', {
      tableName: "session-table2",
      partitionKey: {
        name: "session_id ",
        type: dynamodb.AttributeType.STRING,
      },
    });
  }
}
