package dev.waylon.terminal.boundedcontexts.terminalsession.infrastructure.service

import io.micrometer.core.instrument.Counter
import io.micrometer.core.instrument.Gauge
import io.micrometer.core.instrument.MeterRegistry
import io.micrometer.core.instrument.Tag
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.cancel
import kotlinx.coroutines.launch

/**
 * Terminal Monitoring Service
 * Collects metrics about terminal sessions and processes
 */
class TerminalMonitoringService(private val meterRegistry: MeterRegistry) {
    // 使用共享的协程作用域，避免每次记录指标都创建新协程
    private val scope = CoroutineScope(SupervisorJob() + Dispatchers.Default)
    
    // Counters
    private val sessionCreatedCounter: Counter = meterRegistry.counter("terminal.sessions.created")
    private val sessionTerminatedCounter: Counter = meterRegistry.counter("terminal.sessions.terminated")
    private val processCreatedCounter: Counter = meterRegistry.counter("terminal.processes.created")
    private val processTerminatedCounter: Counter = meterRegistry.counter("terminal.processes.terminated")
    private val bytesWrittenCounter: Counter = meterRegistry.counter("terminal.bytes.written")
    private val bytesReadCounter: Counter = meterRegistry.counter("terminal.bytes.read")
    
    // Gauges
    private var activeSessionsGauge: Gauge? = null
    private var activeProcessesGauge: Gauge? = null

    /**
     * Initialize gauges
     */
    fun initializeGauges(
        activeSessionsProvider: () -> Int,
        activeProcessesProvider: () -> Int
    ) {
        activeSessionsGauge = Gauge.builder("terminal.sessions.active", activeSessionsProvider)
            .description("Number of active terminal sessions")
            .register(meterRegistry)

        activeProcessesGauge = Gauge.builder("terminal.processes.active", activeProcessesProvider)
            .description("Number of active terminal processes")
            .register(meterRegistry)
    }

    /**
     * Increment session created counter
     */
    fun incrementSessionCreated() {
        sessionCreatedCounter.increment()
    }

    /**
     * Increment session terminated counter
     */
    fun incrementSessionTerminated() {
        sessionTerminatedCounter.increment()
    }

    /**
     * Increment process created counter
     */
    fun incrementProcessCreated() {
        processCreatedCounter.increment()
    }

    /**
     * Increment process terminated counter
     */
    fun incrementProcessTerminated() {
        processTerminatedCounter.increment()
    }

    /**
     * Record bytes written to terminal
     */
    fun recordBytesWritten(bytes: Int) {
        // 使用共享的协程作用域记录指标，避免阻塞业务线程
        scope.launch {
            bytesWrittenCounter.increment(bytes.toDouble())
        }
    }
    
    /**
     * Record bytes read from terminal
     */
    fun recordBytesRead(bytes: Int) {
        // 使用共享的协程作用域记录指标，避免阻塞业务线程
        scope.launch {
            bytesReadCounter.increment(bytes.toDouble())
        }
    }
    
    /**
     * Record session duration
     */
    fun recordSessionDuration(durationMs: Long) {
        // 使用共享的协程作用域记录指标，避免阻塞业务线程
        scope.launch {
            meterRegistry.timer("terminal.sessions.duration")
                .record(durationMs, java.util.concurrent.TimeUnit.MILLISECONDS)
        }
    }
    
    /**
     * Record process duration
     */
    fun recordProcessDuration(durationMs: Long) {
        // 使用共享的协程作用域记录指标，避免阻塞业务线程
        scope.launch {
            meterRegistry.timer("terminal.processes.duration")
                .record(durationMs, java.util.concurrent.TimeUnit.MILLISECONDS)
        }
    }
    
    /**
     * Record error count
     */
    fun recordError(errorType: String) {
        // 使用共享的协程作用域记录指标，避免阻塞业务线程
        scope.launch {
            meterRegistry.counter("terminal.errors", listOf(Tag.of("type", errorType)))
                .increment()
        }
    }
    
    /**
     * 关闭监控服务，清理资源
     */
    fun shutdown() {
        // 取消所有协程
        scope.cancel()
    }
}
