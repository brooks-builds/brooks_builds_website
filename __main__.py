"""A Python Pulumi program"""

import pulumi

stack = pulumi.get_stack()

pulumi.export("x", stack)
