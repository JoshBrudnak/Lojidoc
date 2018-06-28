package org.jbrud.gradle.javadoctomd;

import org.gradle.api.Plugin;
import org.gradle.api.Project;

public class JavadocToMdPlugin implements Plugin<Project> {
  public void apply(Project project) {
    project.getTasks().create("generateDocs", JavadocToMd.class, (task) -> {
    });
  }
}
