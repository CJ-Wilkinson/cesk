from Rule import Rule
from Config import Config
from python.cesk.Control import Control
from python.cesk.Environment import Environment
from python.cesk.Store import Store
from python.cesk.Kontinuation import  AssignK, LvalK, Kontinuation
from Value import Value
from AstNode import Assign, Expr, Stmt
from Address import Address

rules = []

# Shared placeholders from the semantic rules.
lval = Expr("lval")
expr = Expr("expr")
succ = Stmt()
kappa = Kontinuation()
rho = Environment()
sigma = Store()
val = Value("val")
addr = Address("addr")


def updated_store(store: Store, address: Address, value: Value) -> Store:
    new_map = dict(store.s)
    new_map[address] = value
    return Store(new_map)


# <Assign(lval, expr), rho, sigma, kappa>
#   -> <expr, rho, sigma, AssignK(lval, succ, kappa)>
rule1_in = Config(Control(Assign(lval, expr)), rho, sigma, kappa)
rule1_out = Config(Control(expr), rho, sigma, AssignK(lval, succ, kappa))
rules.append(Rule(rule1_in, rule1_out))

# <val, rho, sigma, AssignK(lval, succ, kappa)>
#   -> <lval, rho, sigma, LvalK(val, succ, kappa)>
rule2_in = Config(Control(Expr(str(val))), rho, sigma, AssignK(lval, succ, kappa))
rule2_out = Config(Control(lval), rho, sigma, LvalK(val, succ, kappa))
rules.append(Rule(rule2_in, rule2_out))

# <addr, rho, sigma, LvalK(val, succ, kappa)>
#   -> <succ, rho, sigma[addr -> val], kappa>
rule3_in = Config(Control(Expr(str(addr))), rho, sigma, LvalK(val, succ, kappa))
rule3_out = Config(Control(succ), rho, updated_store(sigma, addr, val), kappa)
rules.append(Rule(rule3_in, rule3_out))

for rule in rules:
    print(rule, "\n")
