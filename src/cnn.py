import torch.nn as nn
import torch.nn.functional as F

class ChessCNN(nn.Module):
    def __init__(self):
        super(ChessCNN, self).__init__()

        # We do not add padding. This approach respects the inherent structure of the
        # chessboard and avoids introducing artificial concepts that are not present in
        # the actual game. It allows the model to focus on learning meaningful spatial
        # relationships and patterns that are genuinely indicative of chess strategies.
        #
        # Since we do not add padding, our spatial dimensions will be squashed to
        # a 7x7 versus the original 8x8 layout
        self.conv1 = nn.Conv2d(12, 64, kernel_size=2, padding=0)

        # In the second layer we have a bigger kernel size to capture higher-level patterns
        # and add padding of 1 to keep the spatial resolution from being further decreased
        self.conv2 = nn.Conv2d(64, 124, kernel_size = 3, padding=1)

        # We now move to the fully connected layer. We do not apply a pooling because there is no
        # need for downsampling as our tensors are very reasonable in size
        # Our tensor currently has the following shape: 7x7x124
        #
        # So we will need to flatten the tensor into one dimensional tensor with a size of: 7 * 7 * 124 = 6076
        # This hidden layer will output 2048 neurons. We will afterwards test this number to balance it against under and overfitting
        self.fc1 = nn.Linear(7 * 7 * 124, 2048)

        self.fc2 = nn.Linear(2048, 4096)

    def forward(self, input):
        # Convolutional Layer 1
        output1 = F.relu(self.conv1(input))
        # Convolutional Layer 2
        output2 = F.relu(self.conv2(output1))
        # Flatten Layer
        # We now flatten the layer from (124, 7, 7) to: (7 * 7 * 124 = 6076)
        output3 = output2.view(7202, -1)  # -1 tells PyTorch to infer the correct size --> torch.Size([7202, 6076])
        
        output4 = F.relu(self.fc1(output3))
        logits = self.fc2(output4)

        # Return the raw scores (logits)
        return logits
