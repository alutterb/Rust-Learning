// Architecture file for sentiment analysis model
use tch::{nn, nn::Module, nn::OptimizerConfig, Device, Tensor};

// Create the CNN struct
// Very basic structure of two convolutional layers followed by two ff linear layers
#[derive(Debug)]
pub struct CNN {
    conv1: nn::Conv2D, // type of field in the struct, so uppercase
    conv2: nn::Conv2D,
    fc1: nn::Linear,
    fc2: nn::Linear,
}

// Implement the model - creating the instance and propogating the signal through the network
impl CNN {
    pub fn new(vs: &nn::Path) -> CNN {
        /*
            Creates a new instance of our CNN struct

            # Example
            let vs = nn::VarStore::new(Device::Cpu);
            let model = CNN::new(&vs.root());

            # Parameters
            `vs` - vector store, where the parameters are stored during training and inference

            # Returns
            CNN Struct with defined model architecture
        */

        // funciton call, so conv2d is lower case
        // assume a 28x28x1 (grayscale) image is passed through, with pooling 2x2
        let conv1 = nn::conv2d(vs, 1, 32,
                               5, Default::default()); // Default::default() is catch-all for any other parameters
        let conv2 = nn::conv2d(vs, 32, 64,
                               5, Default::default());
        let fc1 = nn::linear(vs, 1024, 120, Default::default());
        let fc2 = nn::linear(vs, 120, 2, Default::default()); // assume binary classification for pos/neg

        CNN { conv1, conv2, fc1, fc2 }
    }

    pub fn forward(&self, input: &Tensor) -> Tensor {
        /*
            Proprogates input through CNN network

            # Example
            Input: [1, 28, 28]
            ↓ conv1
            [32, 24, 24]
            ↓ Pooling
            [32, 12, 12]
            ↓ conv2
            [64, 8, 8]
            ↓ Pooling
            [64, 4, 4]
            ↓ Flatten
            [1024]
            ↓ fc1
            [128]
            ↓ fc2 (Output)
            [2]


            # Parameters
            `input` - input data that CNN processes to generate an output
                ex: 28x28x1 gray scale image will be a tensor of [batch_size, 1,28,28] where batch_size is number of images processed in a forward pass

            # Returns
            predictions for each class (in this case, positive/negative)
        */
        input
            .view([-1,1,28,28]) // 28x28x1 grayscale input image 
            .apply(&self.conv1) // apply 32 filters with stride 1, and kernel size of 5, so input dimension reduces by 5 - 1 = 4 -> new dimensions = 32 x 24 x 24
            .max_pool2d_default(2) // 2x2 pooling -> input dimensions cut in half so now we have -> 32x12x12
            .relu()
            .apply(&self.conv2) // same reduction, now with 64 filters -> 64x8x8
            .max_pool2d_default(2) // -> 64x4x4
            .relu()
            .view([-1,1024]) // Flatten 64*4*4 = 1024
            .apply(&self.fc1) // -> 128
            .relu()
            .apply(&self.fc2) // -> prediction classes (in this case, 2)
    }
}

