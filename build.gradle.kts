plugins {
    scala
    application
}

repositories {
    mavenCentral()
}

dependencies {
    implementation("org.scala-lang:scala3-library_3:3.3.0-RC4")
}

application {
    mainClass.set("anvil.App")
}
