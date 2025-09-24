import com.google.protobuf.gradle.*
import com.github.jengelman.gradle.plugins.shadow.tasks.ShadowJar

plugins {
    id("java")
    id("com.google.protobuf") version "0.9.5"
    id("com.github.johnrengelman.shadow") version "8.1.1"
}

val grpcVersion = "1.75.0"

group = "com.st"
version = "1.0"

repositories {
    mavenCentral()
}

dependencies {
    implementation(project(":common"))
    implementation("com.google.protobuf:protobuf-java:4.27.2")
    implementation("io.grpc:grpc-netty-shaded:${grpcVersion}")
    implementation("io.grpc:grpc-stub:${grpcVersion}")
    implementation("io.grpc:grpc-protobuf:${grpcVersion}")
    implementation("io.grpc:grpc-services:${grpcVersion}")
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

tasks.named<ShadowJar>("shadowJar") {
    manifest {
        attributes["Main-Class"] = "com.st.client.ClientGUI"
    }
    mergeServiceFiles()
    archiveClassifier.set("") // Creates 'client-1.0.jar' instead of 'client-1.0-all.jar'
}

// Ensure the 'build' task depends on 'shadowJar'
tasks.named("build") {
    dependsOn(tasks.named("shadowJar"))
}

// Disable the standard 'jar' task to avoid creating a confusing, non-executable JAR
tasks.named<Jar>("jar") {
    enabled = false
}