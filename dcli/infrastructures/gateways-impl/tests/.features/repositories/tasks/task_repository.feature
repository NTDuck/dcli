Feature: Task Repository
  Rule: `save()` behaves as expected
    Scenario: S0
      Given a repository with 0 tasks
      When adding 1 task
      Then the repository has 1 task

    Scenario: S1
      Given a repository with 1 task
      When adding 1 task
      Then the repsitory has 2 tasks

    Scenario: S2
      Given a repository with 512 tasks
      When adding 512 tasks
      Then the repository has 512 tasks

  Rule: `remove()` behaves as expected
    Scenario: S0
      Given a repository with 1 task
      When removing 1 task
      Then the repository has 0 tasks

    Scenario: S1
      Given a repository with 0 tasks
      When removing 1 task
      Then the repository has 0 tasks

    Scenario: S2
      Given a repository with 1024 tasks
      When removing 512 tasks
      Then the repository has 512 tasks

  Rule: `clear()` behaves as expected
    Scenario: S0
      Given a repository with 0 tasks
      When clearing all tasks
      Then the repository has 0 tasks

    Scenario: S1
      Given a repository with 1 task
      When clearing all tasks
      Then the repository has 0 tasks

    Scenario: S2
      Given a repository with 1024 tasks
      When clearing all tasks
      Then the repository has 0 tasks
