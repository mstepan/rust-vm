#!/usr/bin/env bash
rm -rf com/max/Hello.class
javac com/max/Hello.java

# java -cp . com.max.Hello

javap -v com/max/Hello.class
