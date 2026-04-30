from nom_parser import parse_document
input = """
            actor BrokenActor {
                States {
                    Spawn:
                        PLAY A 10
                        INVALID_TOKEN_SHOULD_FAIL
                        Loop
                }
            }
"""
try:
    print(parse_document(input))
except Exception as e:
    print(e)
