package dev.waylon.terminal.boundedcontexts.terminalsession.application.useCase

/**
 * Generic Use Case Interface
 * Provides a consistent structure for all use cases in the application
 * Follows the Command Pattern from SOLID principles
 * 
 * @param I Input type for the use case
 * @param O Output type for the use case
 */
interface UseCase<in I, out O> {
    /**
     * Execute the use case with the given input
     * @param input The input data for the use case
     * @return The output result of the use case
     */
    suspend operator fun invoke(input: I): O
}

/**
 * Use Case Interface for operations that don't require input
 * 
 * @param O Output type for the use case
 */
interface NoInputUseCase<out O> {
    /**
     * Execute the use case without input
     * @return The output result of the use case
     */
    suspend operator fun invoke(): O
}