package merkle

import (
	"fmt"
	"github.com/k-kaddal/zama-server/pkg/utils"
)

type MerkleTree struct {
	Root *MerkleNode
}

type MerkleNode struct {
	Hash  string
	Left  *MerkleNode
	Right *MerkleNode
}

func NewMerkleTree(fileContents [][]byte) *MerkleTree {
	var nodes []MerkleNode

	// create leafs
	for _, content := range fileContents {
		hash := utils.Hash(content)
		node := MerkleNode{
			Hash:  hash,
			Left:  nil,
			Right: nil,
		}
		nodes = append(nodes, node)
	}

	// create branches
	for len(nodes) > 1 {
		var newBranch []MerkleNode


		for i := 0; i < len(nodes); i += 2 {
			leftNode := &nodes[i]
			rightNode := leftNode

			//in case of even nodes duplicate the last one
			if i+1 < len(nodes) {
				rightNode = &nodes[i+1]
			}


			node := MerkleNode{
				Hash:  utils.HashNodes(leftNode.Hash, rightNode.Hash),
				Left:  leftNode,
				Right: rightNode,
			}

			newBranch = append(newBranch, node)
		}
		nodes = newBranch
	}

	return &MerkleTree{Root: &nodes[0]}
}

func (tree *MerkleTree) GetMerkleProof(targetHash string) ([]string, error) {
	fmt.Printf("Target Hash: %s \n", targetHash)

	var proof []string

	var getSiblings func(node *MerkleNode) bool
	getSiblings = func(node *MerkleNode) bool {
		if node == nil {
			return false
		}

		if targetHash == node.Hash {
			return true
		}

		leftFound := getSiblings(node.Left)
		if leftFound {
			proof = append(proof, node.Right.Hash)
			return true
		}

		rightFound := getSiblings(node.Right)
		if rightFound {
			proof = append(proof, node.Left.Hash)
			return true
		}

		return false
	}

	getSiblings(tree.Root)

	if len(proof) == 0 {
		return nil, fmt.Errorf("Target hash not found in the Merkle tree")
	}

	return proof, nil
}

