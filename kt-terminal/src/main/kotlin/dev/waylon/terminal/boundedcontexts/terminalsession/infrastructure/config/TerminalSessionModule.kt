package dev.waylon.terminal.boundedcontexts.terminalsession.infrastructure.config

import dev.waylon.terminal.boundedcontexts.terminalsession.application.process.TerminalProcessFactory
import dev.waylon.terminal.boundedcontexts.terminalsession.application.process.TerminalProcessManager
import dev.waylon.terminal.boundedcontexts.terminalsession.application.service.TerminalCommunicationService
import dev.waylon.terminal.boundedcontexts.terminalsession.application.service.TerminalSessionService
import dev.waylon.terminal.boundedcontexts.terminalsession.application.useCase.CreateTerminalSessionUseCase
import dev.waylon.terminal.boundedcontexts.terminalsession.application.useCase.GetAllTerminalSessionsUseCase
import dev.waylon.terminal.boundedcontexts.terminalsession.application.useCase.GetTerminalSessionByIdUseCase
import dev.waylon.terminal.boundedcontexts.terminalsession.application.useCase.ResizeTerminalUseCase
import dev.waylon.terminal.boundedcontexts.terminalsession.application.useCase.TerminateTerminalSessionUseCase
import dev.waylon.terminal.boundedcontexts.terminalsession.domain.TerminalSessionFactory
import dev.waylon.terminal.boundedcontexts.terminalsession.domain.TerminalSessionRepository
import dev.waylon.terminal.boundedcontexts.terminalsession.infrastructure.repository.InMemoryTerminalSessionRepository
import dev.waylon.terminal.boundedcontexts.terminalsession.infrastructure.service.Pty4jTerminalProcessManager
import dev.waylon.terminal.boundedcontexts.terminalsession.infrastructure.service.TerminalConfigService
import org.koin.dsl.module

// Export the terminal session module for use in Koin configuration
val terminalSessionModule = module {

    // Terminal configuration
    single { TerminalConfigService(get()).loadConfig() }

    // Factories
    single { TerminalSessionFactory(get()) }
    single { TerminalProcessFactory(get()) }

    // Session storage
    single<TerminalSessionRepository> { InMemoryTerminalSessionRepository() }

    // Service layer
    single<TerminalSessionService> { TerminalSessionService(get(), get(), get(), get()) }

    // Terminal process manager
    single<TerminalProcessManager> { Pty4jTerminalProcessManager(get(), get()) }

    // Communication service
    single { TerminalCommunicationService(get(), get()) }

    // Use Cases - Business Logic Layer
    single { CreateTerminalSessionUseCase(get()) }
    single { GetAllTerminalSessionsUseCase(get()) }
    single { GetTerminalSessionByIdUseCase(get()) }
    single { ResizeTerminalUseCase(get()) }
    single { TerminateTerminalSessionUseCase(get()) }

}
