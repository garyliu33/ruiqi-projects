import com.google.protobuf.gradle.*

plugins {
    id("java")
    id("com.google.protobuf") version "0.9.5"
}

val grpcVersion = "1.65.0"

group = "com.st"
version = "1.0"

repositories {
    mavenCentral()
}

dependencies {
    implementation("com.google.protobuf:protobuf-java:4.27.2")
    implementation("io.grpc:grpc-stub:${grpcVersion}")
    implementation("io.grpc:grpc-protobuf:${grpcVersion}")
    implementation("javax.annotation:javax.annotation-api:1.3.2")

    testImplementation(platform("org.junit:junit-bom:5.10.0"))
    testImplementation("org.junit.jupiter:junit-jupiter")

    protobuf(files("../../../../SchottenTotten2_proto/protos/"))
}

protobuf {
    protoc {
        artifact = "com.google.protobuf:protoc:3.25.3"
    }
    plugins {
        id("grpc") {
            artifact = "io.grpc:protoc-gen-grpc-java:${grpcVersion}"
        }
    }
    generateProtoTasks {
        ofSourceSet("main").forEach { it.plugins { id("grpc") } }
    }
}

tasks.test {
    useJUnitPlatform()
}