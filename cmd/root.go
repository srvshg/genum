/*
Copyright © 2022 NAME HERE <EMAIL ADDRESS>

*/
package cmd

import (
	"errors"
	"fmt"
	"math/rand"
	"os"
	"time"

	"github.com/spf13/cobra"
)

var upper int
var lower int

// rootCmd represents the base command when called without any subcommands
var rootCmd = &cobra.Command{
	Use:     "genum",
	Short:   "Generate random number of any type",
	Long:    `Genum is command for generating random number created by Sarvesh`,
	Version: "0.1",
	CompletionOptions: cobra.CompletionOptions{
		DisableDefaultCmd: true,
	},
	Args: func(cmd *cobra.Command, args []string) error {
		if upper < 0 {
			return errors.New("Upper limit should be greater than zero")
		}
		if lower < 0 {
			return errors.New("Lower limit should be greater than zero")
		}
		return nil
	},
	Run: func(cmd *cobra.Command, args []string) {
		random(lower, upper)
	},
	// Uncomment the following line if your bare application
	// has an action associated with it:
	// Run: func(cmd *cobra.Command, args []string) { },
}

// Execute adds all child commands to the root command and sets flags appropriately.
// This is called by main.main(). It only needs to happen once to the rootCmd.
func Execute() {
	err := rootCmd.Execute()
	if err != nil {
		os.Exit(1)
	}
}

func init() {
	// Here you will define your flags and configuration settings.
	// Cobra supports persistent flags, which, if defined here,
	// will be global for your application.

	// rootCmd.PersistentFlags().StringVar(&cfgFile, "config", "", "config file (default is $HOME/.genum.yaml)")
	rootCmd.PersistentFlags().IntVarP(&upper, "upper", "u", 1, "upper limit")
	rootCmd.PersistentFlags().IntVarP(&lower, "lower", "l", 0, "lower limit")

	// Cobra also supports local flags, which will only run
	// when this action is called directly.
	rootCmd.Flags().BoolP("toggle", "t", false, "Help message for toggle")
}

func random(min int, max int) {
	rand.Seed(time.Now().UnixNano())
	fmt.Println(rand.Intn(max-min+1) + min)
}
