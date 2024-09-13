ThisBuild / version := "0.1.0-SNAPSHOT"

ThisBuild / scalaVersion := "3.3.3"

lazy val root = (project in file("."))
  .settings(
    name := "ParsePointsJson"
  )

// set fork to true to run the application in a separate JVM
fork := true

// use G1GC garbage collector
javaOptions ++= Seq("-XX:+UseG1GC", "-Xms4G", "-Xmx4G")
