Feature: Task Repository
  Rule: `save()`
    Scenario:
      Given an empty repository
      When adding task 0
      Then the repository only contains task 0

  Rule: `remove()`
    Scenario:
      Given an empty repository
      When removing task 0
      Then the repository is empty

    Scenario:
      Given a repository containing tasks 0 to 1024
      When removing task 444
      Then the repository contains tasks 0 to 1024 except 444
