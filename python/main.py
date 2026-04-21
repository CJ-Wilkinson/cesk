from Rule import Rule
from cesk.Config import Config
from cesk.Control import Control
from cesk.Environment import Environment
from cesk.Store import Store
from cesk.Kontinuation import  AssignK, LvalK, Kontinuation
from cesk.Value import Value
from cesk.AstNode import Assign, Expr, Stmt
from cesk.Address import Address


# <Assign(lval, expr), rho, sigma, kappa>
#   -> <expr, rho, sigma, AssignK(lval, succ, kappa)>
rule1_in = Config(Control(Assign()), Kontinuation())
rule1_out = Config(Control(Expr()), AssignK(succ="succ"))
rule  = Rule(rule1_in, rule1_out)

print(rule)

