import sbt.Keys.testFrameworks

ThisBuild / version          := "0.1.0-SNAPSHOT"
ThisBuild / organization     := "dev.almibe"
ThisBuild / organizationName := "almibe.dev"
ThisBuild / scalaVersion     := "2.13.3"

lazy val root = project
  .in(file("."))
  .settings(
    name := "slonky-in-memory",
    libraryDependencies += "dev.almibe" %% "slonky" % "0.1.0-SNAPSHOT",
    libraryDependencies += "org.scalameta" %% "munit" % "0.7.12" % Test,
    libraryDependencies += "dev.almibe" %% "slonky-test-suite" % "0.1.0-SNAPSHOT" % Test,
    testFrameworks += new TestFramework("munit.Framework")
  )
