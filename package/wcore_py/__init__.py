from .wcore_py import State


def eval(code: str) -> list:
    """Super simple eval command"""

    state = State()
    return state.apply(code)
