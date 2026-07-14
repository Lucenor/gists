# Simplified Representation of Logit Masking via FSM
import torch

def apply_fsm_constraints(logits, current_state, allowed_tokens):
    """
    Forces non-compliant token logits to negative infinity.
    Prerequisites: PyTorch, predefined FSM graph.
    """
    # Create a mask of negative infinity
    mask = torch.full_like(logits, float('-inf'))
    
    # Unmask only tokens permitted by the current FSM state
    for token_id in allowed_tokens[current_state]:
        mask[0, token_id] = 0.0
        
    # Apply mask to logits
    constrained_logits = logits + mask
    return constrained_logits

# Outcome: The model mathematically cannot generate an unapproved structure.
