plugins {
    id("java")
    id("com.google.protobuf") version "0.9.5"
}

group = "com.st"
version = "1.0"

repositories {
    mavenCentral()
}

dependencies {
    implementation("com.google.protobuf:protobuf-java:4.32.0")
    testImplementation(platform("org.junit:junit-bom:5.10.0"))
    testImplementation("org.junit.jupiter:junit-jupiter")

    protobuf(files("../../../../SchottenTotten2_proto/protos/"))
}

tasks.test {
    useJUnitPlatform()
}

tasks.jar {
    manifest {
        attributes["Main-Class"] = "com/st/host/HostGUI" // Replace with your actual class
    }

    // Optional: include dependencies (fat jar)
    from({
        configurations.runtimeClasspath.get().map { if (it.isDirectory) it else zipTree(it) }
    })
    duplicatesStrategy = DuplicatesStrategy.EXCLUDE
}